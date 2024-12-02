use poem::{get, handler, Route};

mod day1;
mod day_2;

#[handler]
fn hello_world() -> &'static str {
    "Hello, bird!"
}
pub fn main_router() -> Route {
    Route::new()
        .at("/", get(hello_world))
        .nest("/-1", day1::route())
        .nest("/2", day_2::route())
}
