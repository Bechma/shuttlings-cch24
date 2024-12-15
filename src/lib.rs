use poem::{get, handler, Route};

mod day1;
mod day_2;
mod day_5;
mod day_9;
mod day_12;

#[handler]
fn hello_world() -> &'static str {
    "Hello, bird!"
}

#[must_use]
pub fn main_router() -> Route {
    Route::new()
        .at("/", get(hello_world))
        .nest("/-1", day1::route())
        .nest("/2", day_2::route())
        .nest("/5", day_5::route())
        .nest("/9", day_9::route())
        .nest("/12", day_12::route())
}
