#![allow(dead_code)]

use poem::http::StatusCode;
use poem::{get, handler, IntoResponse, Response, Route};

#[handler]
fn seek() -> Response {
    StatusCode::FOUND
        .with_header("Location", "https://www.youtube.com/watch?v=9Gc4QTqslN4")
        .into_response()
}

pub fn route() -> Route {
    Route::new().at("/seek", get(seek))
}
