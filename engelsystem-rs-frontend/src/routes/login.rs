use actix_web::{cookie::Cookie, get, http::header::{self, ContentType}, post, web::{self, Data, Form, Html}, HttpResponse, Responder};
use serde::Deserialize;
use snafu::{IntoError, ResultExt};
use tera::{Context, Tera};

use crate::{generated::BackendErr, session::PublicSession, Error};

#[derive(Deserialize)]
struct LoginPageData {
    created: Option<bool>,
}

#[get("/login")]
pub async fn login_page(
    web::Query(data): web::Query<LoginPageData>,
    templates: Data<Tera>,
    session: PublicSession, 
) -> crate::Result<impl Responder> {
    let mut context = Context::new();
    session.base_data("Real Org").insert(&mut context);
    context.insert("created", &data.created.unwrap_or(false));

    let rendered = templates.render("login.html", &context)
        .map_err(|e| {
            tracing::error!("Template error: {e:?}");
            crate::error::generated::TemplateErr.into_error(e)
        })?;
    Ok(Html::new(rendered))
}

#[post("/login")]
pub async fn request_login(
    templates: Data<Tera>,
    client: Data<reqwest::Client>,
    Form(body): Form<serde_json::Value>,
    session: PublicSession,
) -> crate::Result<impl Responder> {
    let mut context = Context::new();
    session.base_data("Real Org").insert(&mut context);

    const LOGIN_URL: &str = "http://127.0.0.1:8081/login";
    let response = client.post(LOGIN_URL)
        .json(&body)
        .send()
        .await
        .context(BackendErr)?;

    if response.status().is_success() {
        let session_id = response.cookies().find(|c| c.name() == "session-id")
            .ok_or_else(|| Error::BackendCookieInvalid { name: "session-id".to_string() })?;

        return Ok(HttpResponse::SeeOther()
            .cookie(
                Cookie::build("session-id", session_id.value())
                .secure(true)
                .http_only(true)
                .finish()
            )
            .append_header((header::LOCATION, "/welcome"))
            .finish());
    }

    let mut context = Context::new();
    session.base_data("Real Org").insert(&mut context);
    context.insert("error", &true);

    let rendered = templates.render("login.html", &context)
        .map_err(|e| {
            tracing::error!("Template error: {e:?}");
            crate::error::generated::TemplateErr.into_error(e)
        })?;

    Ok(HttpResponse::Unauthorized()
        .content_type(ContentType::html())
        .body(rendered))
}
