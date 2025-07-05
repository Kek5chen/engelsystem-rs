use crate::utils::validation::*;
use actix_session::Session;
use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};
use engelsystem_rs_db::{DatabaseConnection, user::verify_user};
use serde::Deserialize;
use tracing::info;
use validator::Validate;
use zeroize::Zeroizing;

#[derive(Debug, Deserialize, Validate)]
struct LoginData {
    #[validate(custom(function = "validate_username"))]
    username: String,
    #[validate(custom(function = "validate_password"))]
    password: Zeroizing<String>,
}

#[post("/login")]
pub async fn request_login(
    Json(data): Json<LoginData>,
    db: Data<DatabaseConnection>,
    session: Session,
) -> crate::Result<impl Responder> {
    let user = verify_user(&data.username, &data.password, &db).await;

    if let Some(user) = user {
        session.clear();
        session.insert("user_id", user.id)?;
        session.insert("role_id", user.role_id)?;

        info!("User {:?} logged in successfully", user.username);

        return Ok(HttpResponse::Ok());
    }

    info!("User {:?} failed to login.", data.username);

    Ok(HttpResponse::Unauthorized())
}
