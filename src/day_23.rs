use poem::http;
use poem::web::Multipart;
use poem_openapi::param::Path;
use poem_openapi::payload::Html;
use serde::Deserialize;

#[derive(Deserialize)]
struct LockfileParsed {
    package: Vec<Packages>,
}

#[derive(Deserialize)]
struct Packages {
    #[serde(default)]
    checksum: Option<String>,
}

pub struct Api;

#[poem_openapi::OpenApi(prefix_path = "/23")]
impl Api {
    #[allow(clippy::unused_async)]
    #[oai(path = "/star", method = "get")]
    async fn star(&self) -> Html<&'static str> {
        Html(r#"<div id="star" class="lit"></div>"#)
    }

    #[allow(clippy::unused_async)]
    #[oai(path = "/present/:color", method = "get")]
    async fn present(&self, Path(color): Path<String>) -> poem::Result<Html<String>> {
        let color_next = match color.as_str() {
            "red" => "blue",
            "blue" => "purple",
            "purple" => "red",
            _ => return Err(poem::Error::from_status(http::StatusCode::IM_A_TEAPOT)),
        };
        let res = format!(
            r#"<div class="present {color}" hx-get="/23/present/{color_next}" hx-swap="outerHTML"><div class="ribbon"></div><div class="ribbon"></div><div class="ribbon"></div><div class="ribbon"></div></div>"#
        );
        Ok(Html(res))
    }

    #[allow(clippy::unused_async)]
    #[oai(path = "/ornament/:state/:n", method = "get")]
    async fn ornament(
        &self,
        Path(state): Path<String>,
        Path(n): Path<String>,
    ) -> poem::Result<Html<String>> {
        let (state, next_state) = match state.as_str() {
            "on" => (" on", "off"),
            "off" => ("", "on"),
            _ => return Err(poem::Error::from_status(http::StatusCode::IM_A_TEAPOT)),
        };
        let n = askama_escape::escape(&n, askama_escape::Html).to_string();
        let res = format!(
            r#"<div class="ornament{state}" id="ornament{n}" hx-trigger="load delay:2s once" hx-get="/23/ornament/{next_state}/{n}" hx-swap="outerHTML"></div>"#
        );
        Ok(Html(res))
    }

    #[oai(path = "/lockfile", method = "post")]
    async fn lockfile(&self, body: Multipart) -> poem::Result<Html<String>> {
        let lockfile = parse_lockfile(body).await?;
        let mut res = Vec::with_capacity(lockfile.package.len());
        for i in lockfile.package.into_iter().filter_map(|p| p.checksum) {
            if i.len() < 10 {
                return Err(err_entity("invalid length"));
            }
            let color = &i[0..6];
            u32::from_str_radix(color, 16).map_err(err_entity)?;
            let top = u8::from_str_radix(&i[6..8], 16).map_err(err_entity)?;
            let left = u8::from_str_radix(&i[8..10], 16).map_err(err_entity)?;
            res.push(format!(
                r#"<div style="background-color:#{color};top:{top}px;left:{left}px;"></div>"#
            ))
        }
        Ok(Html(res.join("\n")))
    }
}

fn err_entity<T: std::fmt::Display>(err: T) -> poem::Error {
    eprintln!("{err}");
    poem::Error::from_status(http::StatusCode::UNPROCESSABLE_ENTITY)
}

fn err_processing<T: std::fmt::Display>(err: T) -> poem::Error {
    eprintln!("{err}");
    poem::Error::from_status(http::StatusCode::BAD_REQUEST)
}

async fn parse_lockfile(mut body: Multipart) -> poem::Result<LockfileParsed> {
    let Some(field) = body.next_field().await.map_err(err_processing)? else {
        return Err(poem::Error::from_status(http::StatusCode::BAD_REQUEST));
    };

    let field = field.text().await.map_err(err_processing)?;

    toml::from_str::<LockfileParsed>(&field).map_err(err_processing)
}
