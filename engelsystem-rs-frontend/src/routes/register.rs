use actix_web::{get, http::header::{self, ContentType}, post, web::{Data, Form, Html}, HttpResponse, Responder};
use snafu::{IntoError, ResultExt};
use tera::{Context, Tera};
use crate::{generated::{BackendErr, TemplateErr}, session::PublicSession};

#[get("/register")]
async fn register_page(templates: Data<Tera>, session: PublicSession) -> crate::Result<impl Responder> {
    let mut context = Context::new();
    session.base_data("Real Org").insert(&mut context);

    let rendered = templates.render("register.html", &context)
        .map_err(|e| {
            tracing::error!("Template error: {e:?}");
            TemplateErr.into_error(e)
        })?;

    Ok(Html::new(rendered))
}

#[post("/register")]
async fn request_register(
    templates: Data<Tera>,
    client: Data<reqwest::Client>,
    Form(body): Form<serde_json::Value>,
    session: PublicSession,
) -> crate::Result<impl Responder> {
    const REGISTER_URL: &str = "http://127.0.0.1:8081/register";
    let response = client.post(REGISTER_URL)
        .json(&body)
        .send()
        .await
        .context(BackendErr)?;

    if response.status().is_success() {
        return Ok(HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/login?created=true"))
            .finish());
    }

    let mut context = Context::new();
    session.base_data("Real Org").insert(&mut context);

    let error = response.text().await.context(BackendErr)?;

    context.insert("errors", &error);

    let rendered = templates.render("register.html", &context)
        .map_err(|e| {
            tracing::error!("Template error: {e:?}");
            TemplateErr.into_error(e)
        })?;

    Ok(HttpResponse::BadRequest()
        .content_type(ContentType::html())
        .body(rendered))
}
