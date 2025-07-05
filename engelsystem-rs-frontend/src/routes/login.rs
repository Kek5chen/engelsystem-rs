use actix_web::{
    HttpResponse, Responder, get,
    http::header::{self},
    post,
    web::{self, Data, Form},
};
use serde::Deserialize;
use snafu::ResultExt;
use tera::Tera;

use crate::{
    Error,
    generated::BackendErr,
    render_template,
    session::{PublicSession, ResponseCookieExt},
    utils::response_ext::ActixResponseExt,
};

#[derive(Deserialize)]
struct LoginPageData {
    created: Option<bool>,
}

#[get("/login")]
pub async fn login_page(
    web::Query(data): web::Query<LoginPageData>,
    templates: Data<Tera>,
    session: PublicSession,
) -> crate::Result<HttpResponse> {
    if session.exists() {
        return Ok(HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/welcome"))
            .finish());
    }

    let rendered = render_template!(&templates, "login.html", session, [ "created" => &data.created.unwrap_or(false) ])?;

    Ok(HttpResponse::Ok().html(rendered))
}

#[post("/login")]
pub async fn request_login(
    templates: Data<Tera>,
    client: Data<reqwest::Client>,
    Form(body): Form<serde_json::Value>,
    session: PublicSession,
) -> crate::Result<impl Responder> {
    const LOGIN_URL: &str = "http://127.0.0.1:8081/login";
    let response = client
        .post(LOGIN_URL)
        .json(&body)
        .send()
        .await
        .context(BackendErr)?;

    if response.status().is_success() {
        let session_id =
            response
                .cookie("session-id")
                .ok_or_else(|| Error::BackendCookieInvalid {
                    name: "session-id".to_string(),
                })?;

        return Ok(HttpResponse::SeeOther()
            .session_cookie(session_id.value())
            .append_header((header::LOCATION, "/welcome"))
            .finish());
    }

    let rendered = render_template!(&templates, "login.html", session, [ "error" => &true ])?;

    Ok(HttpResponse::Unauthorized().html(rendered))
}
