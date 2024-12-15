use cargo_manifest::Manifest;
use poem::web::headers::ContentType;
use poem::web::TypedHeader;
use poem::Body;
use poem_openapi::payload::PlainText;
use poem_openapi::OpenApi;
use serde::Deserialize;

struct SkipErrorVisitor<T>(std::marker::PhantomData<T>);

impl<'de, T> serde::de::Visitor<'de> for SkipErrorVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut values = Vec::new();

        loop {
            match seq.next_element::<T>() {
                Ok(Some(value)) => values.push(value),
                Ok(None) => break,
                Err(_) => continue,
            }
        }

        Ok(values)
    }
}

fn skip_deserialize_errors<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    deserializer.deserialize_seq(SkipErrorVisitor(std::marker::PhantomData))
}

#[derive(Deserialize)]
struct Metadata {
    #[serde(deserialize_with = "skip_deserialize_errors")]
    #[serde(default)]
    orders: Vec<Order>,
}

#[derive(Deserialize)]
struct Order {
    item: String,
    #[serde(default)]
    quantity: Option<u32>,
}

#[derive(Debug, poem_openapi::ApiResponse)]
enum MyResponse {
    #[oai(status = 200)]
    Ok(PlainText<String>),
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 400)]
    BadRequest(PlainText<String>),
    #[oai(status = 415)]
    UnsupportedMediaType,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.item, self.quantity.unwrap_or_default())
    }
}

fn parse_manifest(manifest: Manifest<Metadata>) -> MyResponse {
    let Some(package) = manifest.package else {
        return MyResponse::BadRequest(PlainText("".to_string()));
    };

    if package.keywords.map_or(true, |x| {
        x.as_local()
            .map_or(true, |x| !x.iter().any(|k| k == "Christmas 2024"))
    }) {
        return MyResponse::BadRequest(PlainText("Magic keyword not provided".to_string()));
    }

    let body = package
        .metadata
        .map(|metadata| {
            metadata
                .orders
                .iter()
                .filter(|o| o.quantity.is_some())
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();

    if body.is_empty() {
        MyResponse::NoContent
    } else {
        MyResponse::Ok(PlainText(body))
    }
}

fn parse_error<T: std::fmt::Display>(err: T) -> MyResponse {
    eprintln!("Failed to parse manifest: {err}");
    MyResponse::BadRequest(PlainText("Invalid manifest".to_string()))
}

pub struct Api;

#[OpenApi(prefix_path = "/5")]
impl Api {
    #[oai(path = "/manifest", method = "post")]
    async fn manifest(&self, TypedHeader(ct): TypedHeader<ContentType>, body: Body) -> MyResponse {
        let content_type = ct.to_string();
        if content_type.contains("toml") {
            Manifest::<Metadata>::from_slice_with_metadata(
                body.into_vec().await.unwrap_or_default().as_slice(),
            )
            .map_or_else(parse_error, parse_manifest)
        } else if content_type.contains("json") {
            body.into_json()
                .await
                .map_or_else(parse_error, parse_manifest)
        } else if content_type.contains("yaml") {
            body.into_string().await.map_or_else(parse_error, |x| {
                serde_yml::from_str::<Manifest<Metadata>>(&x)
                    .map_or_else(parse_error, parse_manifest)
            })
        } else {
            MyResponse::UnsupportedMediaType
        }
    }
}
