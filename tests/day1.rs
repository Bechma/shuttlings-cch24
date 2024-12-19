mod helper;
use helper::main_router;
use poem::http::StatusCode;
use poem::test::TestClient;

#[tokio::test]
async fn test_day_minus_1() {
    let res = TestClient::new(main_router()).get("/-1/seek").send().await;
    res.assert_status(StatusCode::FOUND);
    res.assert_header("Location", "https://www.youtube.com/watch?v=9Gc4QTqslN4");
}
