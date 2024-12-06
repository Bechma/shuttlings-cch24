use cargo_manifest::Manifest;
use poem::http::StatusCode;
use poem::web::headers::ContentType;
use poem::web::TypedHeader;
use poem::{handler, post, Body, IntoResponse, Response, Route};
use serde::Deserialize;

struct SkipErrorVisitor<T>(std::marker::PhantomData<T>);

impl<'de, T> serde::de::Visitor<'de> for SkipErrorVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut values = Vec::new();

        loop {
            match seq.next_element::<T>() {
                Ok(Some(value)) => values.push(value),
                Ok(None) => break,
                Err(_) => continue,
            }
        }

        Ok(values)
    }
}

fn skip_deserialize_errors<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    deserializer.deserialize_seq(SkipErrorVisitor(std::marker::PhantomData))
}

#[derive(Deserialize)]
struct Metadata {
    #[serde(deserialize_with = "skip_deserialize_errors")]
    #[serde(default)]
    orders: Vec<Order>,
}

#[derive(Deserialize)]
struct Order {
    item: String,
    #[serde(default)]
    quantity: Option<u32>,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.item, self.quantity.unwrap_or_default())
    }
}

fn parse_manifest(manifest: Manifest<Metadata>) -> Response {
    let Some(package) = manifest.package else {
        return StatusCode::NO_CONTENT.into_response();
    };

    if package.keywords.map_or(true, |x| {
        x.as_local()
            .map_or(true, |x| !x.iter().any(|k| k == "Christmas 2024"))
    }) {
        return StatusCode::BAD_REQUEST
            .with_body("Magic keyword not provided")
            .into_response();
    }

    let body = package
        .metadata
        .map(|metadata| {
            metadata
                .orders
                .iter()
                .filter(|o| o.quantity.is_some())
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();

    if body.is_empty() {
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::OK.with_body(body).into_response()
    }
}

fn parse_error<T: std::fmt::Display>(err: T) -> Response {
    eprintln!("Failed to parse manifest: {err}");
    StatusCode::BAD_REQUEST
        .with_body("Invalid manifest")
        .into_response()
}

#[allow(dead_code)]
#[handler]
async fn manifest_handler(TypedHeader(ct): TypedHeader<ContentType>, body: Body) -> Response {
    let content_type = ct.to_string();
    if content_type.contains("toml") {
        Manifest::<Metadata>::from_slice_with_metadata(
            body.into_vec().await.unwrap_or_default().as_slice(),
        )
        .map_or_else(parse_error, parse_manifest)
    } else if content_type.contains("json") {
        body.into_json()
            .await
            .map_or_else(parse_error, parse_manifest)
    } else if content_type.contains("yaml") {
        body.into_string().await.map_or_else(parse_error, |x| {
            serde_yml::from_str::<Manifest<Metadata>>(&x).map_or_else(parse_error, parse_manifest)
        })
    } else {
        StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response()
    }
}

pub(crate) fn route() -> Route {
    Route::new().at("/manifest", post(manifest_handler))
}
