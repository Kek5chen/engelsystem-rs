use crate::{
    generated::BackendErr, render_template, session::PublicSession,
    utils::response_ext::ActixResponseExt,
};
use actix_web::{
    HttpResponse, Responder, get,
    http::header::{self},
    post,
    web::{Data, Form},
};
use snafu::{IntoError, ResultExt};
use tera::Tera;

#[get("/register")]
async fn register_page(
    templates: Data<Tera>,
    session: PublicSession,
) -> crate::Result<impl Responder> {
    if session.exists() {
        return Ok(HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/welcome"))
            .finish());
    }

    let rendered = render_template!(&templates, "register.html", [])?;

    Ok(HttpResponse::Ok().html(rendered))
}

#[post("/register")]
async fn request_register(
    templates: Data<Tera>,
    client: Data<reqwest::Client>,
    Form(body): Form<serde_json::Value>,
    session: PublicSession,
) -> crate::Result<impl Responder> {
    if session.exists() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    const REGISTER_URL: &str = "http://127.0.0.1:8081/register";
    let response = client
        .post(REGISTER_URL)
        .json(&body)
        .send()
        .await
        .context(BackendErr)?;

    if response.status().is_success() {
        return Ok(HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/login?created=true"))
            .finish());
    }

    let error = response.text().await.context(BackendErr)?;

    let rendered = render_template!(&templates, "register.html",  [ "errors" => &error ])?;

    Ok(HttpResponse::BadRequest().html(rendered))
}
