use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub struct Api {
    pool: sqlx::PgPool,
    tokens: Arc<Mutex<HashSet<String>>>,
}

impl Api {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool,
            tokens: Arc::new(Mutex::new(HashSet::new())),
        }
    }
}

#[derive(Debug, serde::Deserialize, sqlx::FromRow, serde::Serialize, poem_openapi::Object)]
struct Quote {
    id: Uuid,
    author: String,
    quote: String,
    created_at: DateTime<Utc>,
    version: i32,
}

#[derive(poem_openapi::Object)]
struct ModifyQuote {
    author: String,
    quote: String,
}

#[derive(Debug, poem_openapi::ApiResponse)]
enum MyResponse {
    #[oai(status = 200)]
    Ok(Json<Quote>),
    #[oai(status = 404)]
    NotFound,
}

#[derive(Debug, poem_openapi::ApiResponse)]
enum Created {
    #[oai(status = 201)]
    Ok(Json<Quote>),
    #[oai(status = 500)]
    Error,
}

#[derive(Debug, poem_openapi::Object)]
struct List {
    quotes: Vec<Quote>,
    page: i32,
    next_token: Option<String>,
}

#[derive(Debug, poem_openapi::ApiResponse)]
enum ListResponse {
    #[oai(status = 200)]
    Ok(Json<List>),
    #[oai(status = 400)]
    BadRequest,
}

#[poem_openapi::OpenApi(prefix_path = "/19")]
impl Api {
    #[oai(path = "/reset", method = "post")]
    async fn reset(&self) {
        sqlx::query!("DELETE FROM quotes")
            .execute(&self.pool)
            .await
            .unwrap();
    }

    #[oai(path = "/cite/:id", method = "get")]
    async fn cite_id(&self, Path(id): Path<Uuid>) -> MyResponse {
        sqlx::query_as!(Quote, "SELECT * FROM quotes WHERE id=$1", id)
            .fetch_one(&self.pool)
            .await
            .map_or_else(
                |x| {
                    eprintln!("cite_id err {x}");
                    MyResponse::NotFound
                },
                |q| MyResponse::Ok(Json(q)),
            )
    }

    #[oai(path = "/remove/:id", method = "delete")]
    async fn remove_id(&self, Path(id): Path<Uuid>) -> MyResponse {
        sqlx::query_as!(Quote, "DELETE FROM quotes WHERE id=$1 RETURNING *", id)
            .fetch_one(&self.pool)
            .await
            .map_or_else(
                |x| {
                    eprintln!("cite_id err {x}");
                    MyResponse::NotFound
                },
                |q| MyResponse::Ok(Json(q)),
            )
    }

    #[oai(path = "/undo/:id", method = "put")]
    async fn undo_id(&self, Path(id): Path<Uuid>, Json(req): Json<ModifyQuote>) -> MyResponse {
        sqlx::query_as!(
            Quote,
            "UPDATE quotes SET author=$2, quote=$3, version=version+1 WHERE id=$1 RETURNING *",
            id,
            req.author,
            req.quote
        )
        .fetch_one(&self.pool)
        .await
        .map_or_else(
            |x| {
                eprintln!("cite_id err {x}");
                MyResponse::NotFound
            },
            |q| MyResponse::Ok(Json(q)),
        )
    }

    #[oai(path = "/draft", method = "post")]
    async fn draft(&self, Json(req): Json<ModifyQuote>) -> Created {
        sqlx::query_as!(
            Quote,
            "INSERT INTO quotes (id, author, quote) VALUES (gen_random_uuid(), $1, $2) RETURNING *",
            req.author,
            req.quote
        )
        .fetch_one(&self.pool)
        .await
        .map_or_else(
            |x| {
                eprintln!("cite_id err {x}");
                Created::Error
            },
            |q| Created::Ok(Json(q)),
        )
    }

    #[oai(path = "/list", method = "get")]
    async fn list(&self, Query(token): Query<Option<String>>) -> ListResponse {
        let (created_at, page) = match token {
            Some(token) => {
                if !self.tokens.lock().unwrap().contains(token.as_str()) {
                    return ListResponse::BadRequest;
                }
                match decode_token(&token) {
                    Ok(token) => token,
                    Err(_) => return ListResponse::BadRequest,
                }
            }
            None => (DateTime::<Utc>::from_timestamp(0, 0).unwrap(), 0),
        };
        sqlx::query_as!(
            Quote,
            r#"SELECT * FROM quotes WHERE created_at >= $1 ORDER BY created_at LIMIT 4"#,
            created_at,
        )
        .fetch_all(&self.pool)
        .await
        .map_or_else(
            |x| {
                eprintln!("cite_id err {x}");
                ListResponse::Ok(Json(List {
                    quotes: vec![],
                    page: 0,
                    next_token: None,
                }))
            },
            move |mut quotes| {
                let next_token = if quotes.len() == 4 {
                    let token = quotes.pop().map(|x| encode_token(x.created_at, page + 1));
                    self.tokens.lock().unwrap().insert(token.clone().unwrap());
                    token
                } else {
                    None
                };
                ListResponse::Ok(Json(List {
                    quotes,
                    page: page + 1,
                    next_token,
                }))
            },
        )
    }
}

const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

fn encode_token(timestamp: DateTime<Utc>, page: i32) -> String {
    let mut token = Vec::with_capacity(12);
    token.extend_from_slice(&timestamp.timestamp_nanos_opt().unwrap().to_be_bytes());
    token.extend_from_slice(&page.to_be_bytes());
    CUSTOM_ENGINE.encode(token)
}

fn decode_token(token: &str) -> Result<(DateTime<Utc>, i32), &'static str> {
    let decoded = CUSTOM_ENGINE.decode(token).map_err(|_| "Invalid token")?;
    let ts = i64::from_be_bytes(decoded[0..8].try_into().map_err(|_| "Invalid ts")?);
    let page = i32::from_be_bytes(decoded[8..12].try_into().map_err(|_| "Invalid page")?);
    Ok((DateTime::from_timestamp_nanos(ts).with_timezone(&Utc), page))
}

mod test {
    #[test]
    fn test_decode_token() {
        super::decode_token("GBKv8VuuLZgAAAAAAAAAAg").unwrap();
    }
}
