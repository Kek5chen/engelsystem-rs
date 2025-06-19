use actix_web::{
    Responder, get,
    web::{self, Data, Html},
};
use snafu::ResultExt;
use tera::Tera;

use crate::{
    generated::BackendErr,
    render_template,
    session::{RequestSessionExt, Session},
};

#[get("/users")]
pub async fn user_list(
    templates: Data<Tera>,
    client: Data<reqwest::Client>,
    session: Session,
) -> crate::Result<impl Responder> {
    const USERS_URL: &str = "http://127.0.0.1:8081/users";
    let users: serde_json::Value = client
        .get(USERS_URL)
        .add_session(&session)
        .send()
        .await
        .context(BackendErr)?
        .json()
        .await
        .context(BackendErr)?;

    let rendered = render_template!(&templates, "user_list.html", session, [
        "users" => &users,
        "logged_in" => &true
    ])?;

    Ok(Html::new(rendered))
}

#[get("/users/{user_id}")]
pub async fn view_user(
    templates: Data<Tera>,
    user_id: web::Path<String>,
    client: Data<reqwest::Client>,
    session: Session,
) -> crate::Result<impl Responder> {
    const USERS_URL: &str = "http://127.0.0.1:8081/users";
    let users: serde_json::Value = client
        .get(format!("{USERS_URL}/{user_id}"))
        .add_session(&session)
        .send()
        .await
        .context(BackendErr)?
        .json()
        .await
        .context(BackendErr)?;

    let rendered = render_template!(&templates, "user_view.html", session, [
        "user" => &users,
        "logged_in" => &true
    ])?;

    Ok(Html::new(rendered))
}
