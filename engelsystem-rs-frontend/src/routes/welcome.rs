use actix_web::{get, web::{Data, Html}, HttpRequest, Responder};
use reqwest::header;
use snafu::{IntoError, ResultExt};
use tera::{Context, Tera};

use crate::{generated::BackendErr, Error};

#[get("/welcome")]
async fn welcome_page(
    templates: Data<Tera>,
    client: Data<reqwest::Client>,
    req: HttpRequest,
) -> crate::Result<impl Responder> {
    let mut context = Context::new();
    
    let Some(session_id) = req.cookie("session-id") else {
        return Err(Error::Unauthorized);
    };

    const USER_URL: &str = "http://127.0.0.1:8081/me";
    let user: serde_json::Value = client.get(USER_URL)
        .header(header::COOKIE, format!("session-id={}", session_id.value()))
        .send()
        .await
        .context(BackendErr)?
        .error_for_status()
        .context(BackendErr)?
        .json()
        .await
        .context(BackendErr)?;

    context.insert("org", "Real Org");
    context.insert("user", &user);
    context.insert("logged_in", &true);

    Ok(Html::new(
        templates.render("welcome.html", &context)
            .map_err(|e| {
                tracing::error!("Template error: {e:?}");
                crate::error::generated::TemplateErr.into_error(e)
            })?
    ))
}

