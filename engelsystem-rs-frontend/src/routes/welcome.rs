use actix_web::{
    Responder, get,
    web::{Data, Html},
};
use reqwest::header;
use snafu::ResultExt;
use tera::Tera;

use crate::{generated::BackendErr, render_template, session::Session};

#[get("/welcome")]
async fn welcome_page(
    templates: Data<Tera>,
    client: Data<reqwest::Client>,
    session: Session,
) -> crate::Result<impl Responder> {
    const USER_URL: &str = "http://127.0.0.1:8081/me";
    let user: serde_json::Value = client
        .get(USER_URL)
        .header(header::COOKIE, session.cookie())
        .send()
        .await
        .context(BackendErr)?
        .error_for_status()
        .context(BackendErr)?
        .json()
        .await
        .context(BackendErr)?;

    Ok(Html::new(
        render_template!(&templates, "welcome.html", session, [ "user" => &user ])?,
    ))
}
