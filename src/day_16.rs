use poem_openapi::payload::Json;
use jsonwebtoken::errors::ErrorKind;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    company: String,
}

#[derive(Debug, poem_openapi::ApiResponse)]
#[oai(header(name = "Set-Cookie", ty = "String"))]
enum WrapResponse {
    #[oai(status = 200)]
    Ok,
}

#[derive(Debug, poem_openapi::ApiResponse)]
enum UnwrapResponse {
    #[oai(status = 200)]
    Ok(Json<serde_json::Value>),
    #[oai(status = 400)]
    BadRequest,
}

#[derive(Debug, poem_openapi::ApiResponse)]
enum DecodeResponse {
    #[oai(status = 200)]
    Ok(Json<serde_json::Value>),
    #[oai(status = 400)]
    BadRequest,
    #[oai(status = 401)]
    Unauthorized,
}

const SECRET_KEY: &[u8] = b"secret";

pub struct Api {
    task1: jsonwebtoken::Validation,
    task2: jsonwebtoken::Validation,
    task2_decode: jsonwebtoken::DecodingKey,
}

impl Api {
    pub fn new() -> Self {
        let mut task1 = jsonwebtoken::Validation::default();
        task1.required_spec_claims.clear();
        let task2_decode =
            jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("day_16_public_key.pem"))
                .unwrap();
        let mut task2 = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
        task2.required_spec_claims.clear();
        task2.algorithms.push(jsonwebtoken::Algorithm::RS512);
        Self {
            task1,
            task2,
            task2_decode,
        }
    }
}

#[poem_openapi::OpenApi(prefix_path = "/16")]
impl Api {
    #[allow(clippy::unused_async)]
    #[oai(path = "/wrap", method = "post")]
    async fn wrap(
        &self,
        Json(body): Json<serde_json::Value>,
        cookie_jar: &poem::web::cookie::CookieJar,
    ) -> WrapResponse {
        match jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &dbg!(body),
            &jsonwebtoken::EncodingKey::from_secret(SECRET_KEY),
        ) {
            Ok(token) => cookie_jar.add(poem::web::cookie::Cookie::new_with_str("gift", token)),
            Err(err) => println!("something went wrong {err}"),
        };
        WrapResponse::Ok
    }

    #[allow(clippy::unused_async)]
    #[oai(path = "/unwrap", method = "get")]
    async fn unwrap(
        &self,
        poem_openapi::param::Cookie(gift): poem_openapi::param::Cookie<String>,
    ) -> UnwrapResponse {
        match jsonwebtoken::decode::<serde_json::Value>(
            &dbg!(gift),
            &jsonwebtoken::DecodingKey::from_secret(SECRET_KEY),
            &self.task1,
        ) {
            Ok(token) => UnwrapResponse::Ok(Json(token.claims)),
            Err(err) => {
                println!("something went wrong {err}");
                UnwrapResponse::BadRequest
            }
        }
    }

    #[allow(clippy::unused_async)]
    #[oai(path = "/decode", method = "post")]
    async fn decode(&self, body: String) -> DecodeResponse {
        match jsonwebtoken::decode::<serde_json::Value>(
            &body,
            &self.task2_decode,
            &self.task2,
        ) {
            Ok(token) => DecodeResponse::Ok(Json(token.claims)),
            Err(err) => match err.kind() {
                ErrorKind::InvalidSignature => DecodeResponse::Unauthorized,
                _ => DecodeResponse::BadRequest,
            },
        }
    }
}
