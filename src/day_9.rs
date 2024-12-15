use leaky_bucket::RateLimiter;
use poem::http::header::CONTENT_TYPE;
use poem::http::StatusCode;
use poem::middleware::AddData;
use poem::web::{Data, Json};
use poem::{
    handler, post, Endpoint, EndpointExt, Error, FromRequest, IntoResponse, Request, RequestBody,
    Response, Route,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const MAX_LITERS: usize = 5;

#[derive(Deserialize, Default, Serialize)]
struct Conversion {
    #[serde(skip_serializing_if = "Option::is_none")]
    liters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gallons: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    litres: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pints: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default = "no_body")]
    has_body: Option<()>,
}

#[allow(clippy::unnecessary_wraps)]
fn no_body() -> Option<()> {
    Some(())
}

impl IntoResponse for Conversion {
    fn into_response(self) -> Response {
        match (
            self.has_body,
            self.liters,
            self.gallons,
            self.litres,
            self.pints,
        ) {
            (Some(()), Some(liters), None, None, None) => Json(Conversion {
                gallons: Some(liters * 0.264_172_9),
                ..Default::default()
            })
            .into_response(),
            (Some(()), None, Some(gallons), None, None) => Json(Conversion {
                liters: Some(gallons * 3.7854),
                ..Default::default()
            })
            .into_response(),
            (Some(()), None, None, Some(litres), None) => Json(Conversion {
                pints: Some(litres * 1.759_753_986_4),
                ..Default::default()
            })
            .into_response(),
            (Some(()), None, None, None, Some(pints)) => Json(Conversion {
                litres: Some(pints * 0.568_261_25),
                ..Default::default()
            })
            .into_response(),
            (None, None, None, None, None) => {
                StatusCode::OK.with_body("Milk withdrawn\n").into_response()
            }
            _ => StatusCode::BAD_REQUEST.into_response(),
        }
    }
}

#[handler]
async fn milk(
    Data(rate_limit): Data<&Arc<Mutex<RateLimiter>>>,
    req: &Request,
    body: poem::Body,
) -> Response {
    if !rate_limit.lock().unwrap().try_acquire(1) {
        return StatusCode::TOO_MANY_REQUESTS
            .with_body("No milk available\n")
            .into_response();
    }

    if let Some(ct) = req.headers().get(CONTENT_TYPE) {
        if let Ok(ct) = ct.to_str() {
            if !ct.starts_with("application/json") {
                return Conversion::default().into_response();
            }
        }
    } else {
        return Conversion::default().into_response();
    }
    let mut body = RequestBody::new(body);
    Json::<Conversion>::from_request(req, &mut body)
        .await
        .map_or_else(Error::into_response, |Json(x)| x.into_response())
}

#[handler]
fn refill(Data(rate_limit): Data<&Arc<Mutex<RateLimiter>>>) {
    *rate_limit.lock().unwrap() = rate_limit_builder();
}

fn rate_limit_builder() -> RateLimiter {
    RateLimiter::builder()
        .max(MAX_LITERS)
        .interval(Duration::from_secs(1))
        .initial(MAX_LITERS)
        .refill(1)
        .build()
}

pub(crate) fn route() -> impl Endpoint {
    // Mutex is not needed, but as the RateLimiter library does not have a manual refilling,
    // we need to replace the whole object:
    // https://github.com/udoprog/leaky-bucket/issues/17
    let rate_limit = Arc::new(Mutex::new(rate_limit_builder()));
    Route::new()
        .at("/milk", post(milk))
        .at("/refill", post(refill))
        .with(AddData::new(rate_limit))
}
