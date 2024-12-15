use poem_openapi::OpenApi;

pub struct Api;

#[derive(Debug, poem_openapi::ApiResponse)]
enum MyResponse {
    #[oai(status = 302)]
    Ok(#[oai(header = "Location")] String),
}

#[OpenApi(prefix_path = "/-1")]
impl Api {
    #[allow(clippy::unused_async)]
    #[oai(path = "/seek", method = "get")]
    async fn seek(&self) -> MyResponse {
        MyResponse::Ok("https://www.youtube.com/watch?v=9Gc4QTqslN4".to_string())
    }
}
