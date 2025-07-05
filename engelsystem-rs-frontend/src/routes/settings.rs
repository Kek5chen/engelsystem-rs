use actix_web::{
    HttpResponse, Responder, get,
    http::header,
    post,
    web::{Data, Form, Html, Query},
};
use engelsystem_rs_db::UserView;
use serde::{Deserialize, Serialize};
use tera::Tera;

use crate::{
    render_template,
    session::{RequestSessionExt, Session},
};

#[derive(Debug, Deserialize)]
pub struct SettingsUpdateStatus {
    success: Option<bool>,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsUpdateRequest {
    username: String,
    email: String,
    password: Option<String>,
    confirm_password: Option<String>,
}

#[get("/settings")]
pub async fn settings_page(
    templates: Data<Tera>,
    client: Data<reqwest::Client>,
    session: Session,
    update_status: Query<SettingsUpdateStatus>,
) -> crate::Result<impl Responder> {
    const USER_URL: &str = "http://127.0.0.1:8081/me";
    let user: UserView = client
        .get(USER_URL)
        .add_session(&session)
        .send()
        .await?
        .json()
        .await?;

    if update_status.success.is_some() {
        Ok(Html::new(
            render_template!(&templates, "settings_updated.html", session, [
                "user" => &user,
                "success" => &update_status.success,
                "error" => &update_status.error
            ])?,
        ))
    } else {
        Ok(Html::new(
            render_template!(&templates, "settings.html", session, [
                "user" => &user
            ])?,
        ))
    }
}

#[post("/settings")]
pub async fn update_settings(
    client: Data<reqwest::Client>,
    session: Session,
    new_settings: Form<SettingsUpdateRequest>,
) -> crate::Result<impl Responder> {
    const UPDATE_SETTINGS: &str = "http://127.0.0.1:8081/settings";
    let response = client
        .post(UPDATE_SETTINGS)
        .add_session(&session)
        .json(&new_settings)
        .send()
        .await?;

    if response.status().is_success() {
        dbg!(response.status());
        Ok(HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/settings?success=true"))
            .finish())
    } else {
        let error = response.text().await?;
        Ok(HttpResponse::SeeOther()
            .append_header((
                header::LOCATION,
                format!("/settings?success=false&error={error}"),
            ))
            .finish())
    }
}
