mod helper;
use helper::main_router;
use poem::http::StatusCode;
use poem::test::TestClient;

async fn t(path: &str, content_type: &str, body: &str, status: StatusCode, response: Option<&str>) {
    let res = TestClient::new(main_router())
        .post(path)
        .content_type(content_type)
        .body(body.to_string())
        .send()
        .await;
    res.assert_status(status);
    if let Some(response) = response {
        res.assert_text(response).await;
    }
}

#[tokio::test]
async fn test_day5_task1_1() {
    t(
        "/5/manifest",
        "application/toml",
        r#"
[package]
name = "not-a-gift-order"
authors = ["Not Santa"]
keywords = ["Christmas 2024"]

[[package.metadata.orders]]
item = "Toy car"
quantity = 2

[[package.metadata.orders]]
item = "Lego brick"
quantity = 230
"#,
        StatusCode::OK,
        Some("Toy car: 2\nLego brick: 230"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task1_2() {
    t(
        "/5/manifest",
        "application/toml",
        r#"
[package]
name = "coal-in-a-bowl"
authors = ["H4CK3R_13E7"]
keywords = ["Christmas 2024"]

[[package.metadata.orders]]
item = "Coal"
quantity = "Hahaha get rekt"
"#,
        StatusCode::NO_CONTENT,
        None,
    )
    .await
}

#[tokio::test]
async fn test_day5_task1_3() {
    t(
        "/5/manifest",
        "application/toml",
        r#"
[package]
name = "coal-in-a-bowl"
authors = ["H4CK3R_13E7"]
keywords = ["Christmas 2024"]

package.metadata.orders = []
"#,
        StatusCode::NO_CONTENT,
        None,
    )
    .await
}

#[tokio::test]
async fn test_day5_task1_4() {
    t(
        "/5/manifest",
        "application/toml",
        r#"
[package]
name = "not-a-gift-order"
authors = ["Not Santa"]
keywords = ["Christmas 2024"]

[[package.metadata.orders]]
item = "Toy car"
quantity = 2

[[package.metadata.orders]]
item = "Lego brick"
quantity = 1.5

[[package.metadata.orders]]
item = "Doll"
quantity = 2

[[package.metadata.orders]]
quantity = 5
item = "Cookie:::\n"

[[package.metadata.orders]]
item = "Thing"
count = 3
"#,
        StatusCode::OK,
        Some("Toy car: 2\nDoll: 2\nCookie:::\n: 5"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task2_1() {
    t(
        "/5/manifest",
        "application/toml",
        r#"
[package]
name = false
authors = ["Not Santa"]
keywords = ["Christmas 2024"]
"#,
        StatusCode::BAD_REQUEST,
        Some("Invalid manifest"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task2_2() {
    t(
        "/5/manifest",
        "application/toml",
        r#"
[package]
name = "not-a-gift-order"
authors = ["Not Santa"]
keywords = ["Christmas 2024"]

[profile.release]
incremental = "stonks"
"#,
        StatusCode::BAD_REQUEST,
        Some("Invalid manifest"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task2_3() {
    t(
        "/5/manifest",
        "application/toml",
        r#"
[package]
name = "big-chungus"
version = "2.0.24"
edition = "2024"
resolver = "2"
readme.workspace = true
keywords = ["Christmas 2024"]

[dependencies]
shuttle-runtime = "1.0.0+when"

[target.shuttlings.dependencies]
cch24-validator = "5+more"

[profile.release]
incremental = false

[package.metadata.stuff]
thing = ["yes", "no"]
"#,
        StatusCode::NO_CONTENT,
        None,
    )
    .await
}

#[tokio::test]
async fn test_day5_task2_4() {
    t(
        "/5/manifest",
        "application/toml",
        r#"
[package]
name = "chig-bungus"
edition = "2023"

[workspace.dependencies]
shuttle-bring-your-own-cloud = "0.0.0"
"#,
        StatusCode::BAD_REQUEST,
        Some("Invalid manifest"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task2_5() {
    t(
        "/5/manifest",
        "application/toml",
        r#"
[package]
name = "chig-bungus"

[workspace]
resolver = "135"

[workspace.dependencies]
shuttle-bring-your-own-cloud = "0.0.0"
"#,
        StatusCode::BAD_REQUEST,
        Some("Invalid manifest"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task3_1() {
    t(
        "/5/manifest",
        "application/toml",
        r#"
[package]
name = "grass"
authors = ["A vegan cow"]
keywords = ["Moooooo"]
"#,
        StatusCode::BAD_REQUEST,
        Some("Magic keyword not provided"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task3_2() {
    t(
        "/5/manifest",
        "application/toml",
        r#"
[package]
name = "chig-bungus"

[workspace]
resolver = "2"

[workspace.dependencies]
shuttle-bring-your-own-cloud = "0.0.0"
"#,
        StatusCode::BAD_REQUEST,
        Some("Magic keyword not provided"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task3_3() {
    t(
        "/5/manifest",
        "application/toml",
        r#"
[package]
name = "slurp"
authors = ["A crazy cow"]
keywords = ["MooOooooooOOOOoo00oo=oOooooo", "Mew", "Moh", "Christmas 2024"]
metadata.orders = [{ item = "Milk 🥛", quantity = 1 }]
"#,
        StatusCode::OK,
        Some("Milk 🥛: 1"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task3_4() {
    t(
        "/5/manifest",
        "application/toml",
        r#"
[package]
name = "snow"
authors = ["The Cow of Christmas"]
keywords = ["Moooooo Merry Christmas 2024"]
"#,
        StatusCode::BAD_REQUEST,
        Some("Magic keyword not provided"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task4_1() {
    t(
        "/5/manifest",
        "text/html",
        r#"<h1>Hello, bird!</h1>"#,
        StatusCode::UNSUPPORTED_MEDIA_TYPE,
        None,
    )
    .await
}

#[tokio::test]
async fn test_day5_task4_2() {
    t(
        "/5/manifest",
        "application/yaml",
        r#"
package:
  name: big-chungus-sleigh
  version: "2.0.24"
  metadata:
    orders:
      - item: "Toy train"
        quantity: 5
      - item: "Toy car"
        quantity: 3
  rust-version: "1.69"
  keywords:
    - "Christmas 2024"
"#,
        StatusCode::OK,
        Some("Toy train: 5\nToy car: 3"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task4_3() {
    t(
        "/5/manifest",
        "application/yaml",
        r#"
package:
  name: big-chungus-sleigh
  metadata:
    orders:
      - item: "Toy train"
        quantity: 5
      - item: "Coal"
      - item: "Horse"
        quantity: 2
  keywords:
    - "Christmas 2024"
"#,
        StatusCode::OK,
        Some("Toy train: 5\nHorse: 2"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task4_4() {
    t(
        "/5/manifest",
        "application/yaml",
        r#"
package:
  name: big-chungus-sleigh
  metadata:
    orders:
      - item: "Toy train"
        quantity: 5
  rust-version: true
  keywords:
    - "Christmas 2024"
"#,
        StatusCode::BAD_REQUEST,
        Some("Invalid manifest"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task4_5() {
    t(
        "/5/manifest",
        "application/json",
        r#"
{
  "package": {
    "name": "big-chungus-sleigh",
    "version": "2.0.24",
    "metadata": {
      "orders": [
        {
          "item": "Toy train",
          "quantity": 5
        },
        {
          "item": "Toy car",
          "quantity": 3
        }
      ]
    },
    "rust-version": "1.69",
    "keywords": [
      "Christmas 2024"
    ]
  }
}
"#,
        StatusCode::OK,
        Some("Toy train: 5\nToy car: 3"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task4_6() {
    t(
        "/5/manifest",
        "application/yaml",
        r#"
{
  "package": {
    "name": "big-chungus-sleigh",
    "metadata": {
      "orders": [
        {
          "item": "Toy train",
          "quantity": 5
        },
        {
          "item": "Coal"
        },
        {
          "item": "Horse",
          "quantity": 2
        }
      ]
    },
    "keywords": [
      "Christmas 2024"
    ]
  }
}
"#,
        StatusCode::OK,
        Some("Toy train: 5\nHorse: 2"),
    )
    .await
}

#[tokio::test]
async fn test_day5_task4_7() {
    t(
        "/5/manifest",
        "application/json",
        r#"
{
  "package": {
    "name": "big-chungus-sleigh",
    "metadata": {
      "orders": [
        {
          "item": "Toy train",
          "quantity": 5
        }
      ]
    }
  }
}
"#,
        StatusCode::BAD_REQUEST,
        Some("Magic keyword not provided"),
    )
    .await
}
