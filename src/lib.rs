use poem::endpoint::StaticFilesEndpoint;
use poem::Route;
use poem_openapi::payload::PlainText;
use poem_openapi::{OpenApi, OpenApiService};

mod day1;
mod day_12;
mod day_16;
mod day_19;
mod day_2;
mod day_23;
mod day_5;
mod day_9;

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello, bird!")
    }
}

#[must_use]
pub fn main_router(pool: sqlx::PgPool) -> Route {
    let oapi = OpenApiService::new(
        (
            Api,
            day1::Api,
            day_2::Api,
            day_5::Api,
            day_16::Api::new(),
            day_19::Api::new(pool),
            day_23::Api,
        ),
        "Shuttling-cch24",
        "1.0",
    );
    let swagger_ui = oapi.swagger_ui();
    Route::new()
        .nest("/", oapi)
        .nest("/9", day_9::route())
        .nest("/12", day_12::route())
        .nest("/swagger", swagger_ui)
        .nest("/assets", StaticFilesEndpoint::new("assets"))
}
