mod helper;
use helper::main_router;
use poem::test::TestClient;

#[tokio::test]
async fn test_day2_task1_1() {
    let res = TestClient::new(main_router())
        .get("/2/dest?from=10.0.0.0&key=1.2.3.255")
        .send()
        .await;
    res.assert_status_is_ok();
    res.assert_text("11.2.3.255").await;
}

#[tokio::test]
async fn test_day2_task1_2() {
    let res = TestClient::new(main_router())
        .get("/2/dest?from=128.128.33.0&key=255.0.255.33")
        .send()
        .await;
    res.assert_status_is_ok();
    res.assert_text("127.128.32.33").await;
}

#[tokio::test]
async fn test_day2_task1_3() {
    let res = TestClient::new(main_router())
        .get("/2/dest?from=192.168.0.1&key=72.96.8.7")
        .send()
        .await;
    res.assert_status_is_ok();
    res.assert_text("8.8.8.8").await;
}

#[tokio::test]
async fn test_day2_task2_1() {
    let res = TestClient::new(main_router())
        .get("/2/key?from=10.0.0.0&to=11.2.3.255")
        .send()
        .await;
    res.assert_status_is_ok();
    res.assert_text("1.2.3.255").await;
}

#[tokio::test]
async fn test_day2_task2_2() {
    let res = TestClient::new(main_router())
        .get("/2/key?from=128.128.33.0&to=127.128.32.33")
        .send()
        .await;
    res.assert_status_is_ok();
    res.assert_text("255.0.255.33").await;
}

#[tokio::test]
async fn test_day2_task2_3() {
    let res = TestClient::new(main_router())
        .get("/2/key?from=192.168.0.1&to=8.8.8.8")
        .send()
        .await;
    res.assert_status_is_ok();
    res.assert_text("72.96.8.7").await;
}

#[tokio::test]
async fn test_day2_task3_dest_1() {
    let res = TestClient::new(main_router())
        .get("/2/v6/dest?from=fe80::1&key=5:6:7::3333")
        .send()
        .await;
    res.assert_status_is_ok();
    res.assert_text("fe85:6:7::3332").await;
}

#[tokio::test]
async fn test_day2_task3_dest_2() {
    let res = TestClient::new(main_router())
        .get("/2/v6/dest?from=aaaa:0:0:0::aaaa&key=ffff:ffff:c:0:0:c:1234:ffff")
        .send()
        .await;
    res.assert_status_is_ok();
    res.assert_text("5555:ffff:c::c:1234:5555").await;
}

#[tokio::test]
async fn test_day2_task3_dest_3() {
    let res = TestClient::new(main_router())
        .get("/2/v6/dest?from=feed:beef:deaf:bad:cafe::&key=::dab:bed:ace:dad")
        .send()
        .await;
    res.assert_status_is_ok();
    res.assert_text("feed:beef:deaf:bad:c755:bed:ace:dad").await;
}

#[tokio::test]
async fn test_day2_task3_key_1() {
    let res = TestClient::new(main_router())
        .get("/2/v6/key?from=fe80::1&to=fe85:6:7::3332")
        .send()
        .await;
    res.assert_status_is_ok();
    res.assert_text("5:6:7::3333").await;
}

#[tokio::test]
async fn test_day2_task3_key_2() {
    let res = TestClient::new(main_router())
        .get("/2/v6/key?from=aaaa::aaaa&to=5555:ffff:c:0:0:c:1234:5555")
        .send()
        .await;
    res.assert_status_is_ok();
    res.assert_text("ffff:ffff:c::c:1234:ffff").await;
}

#[tokio::test]
async fn test_day2_task3_key_3() {
    let res = TestClient::new(main_router())
        .get("/2/v6/key?from=feed:beef:deaf:bad:cafe::&to=feed:beef:deaf:bad:c755:bed:ace:dad")
        .send()
        .await;
    res.assert_status_is_ok();
    res.assert_text("::dab:bed:ace:dad").await;
}
