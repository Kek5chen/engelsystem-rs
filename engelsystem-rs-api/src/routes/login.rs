use crate::utils::schema_impls::ZeroizingDef;
use crate::{Error, utils::validation::*};
use actix_session::Session;
use actix_web::web::{Data, Json};
use apistos::{ApiComponent, actix::NoContent, api_operation};
use engelsystem_rs_db::{DatabaseConnection, user::verify_user};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::info;
use validator::Validate;
use zeroize::Zeroizing;

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct LoginData {
    #[validate(custom(function = "validate_username"))]
    username: String,
    #[validate(custom(function = "validate_password"))]
    #[serde(with = "ZeroizingDef::<String>")]
    password: Zeroizing<String>,
}

#[api_operation(
    tag = "account",
    summary = "Request to log in with the given credentials",
    skip_args = "session"
)]
pub async fn request_login(
    Json(data): Json<LoginData>,
    db: Data<DatabaseConnection>,
    session: Session,
) -> crate::Result<NoContent> {
    let user = verify_user(&data.username, &data.password, &db).await;

    if let Some(user) = user {
        session.clear();
        session.insert("user_id", user.id)?;
        session.insert("role_id", user.role_id)?;

        info!("User {:?} logged in successfully", user.username);

        return Ok(NoContent);
    }

    info!("User {:?} failed to login.", data.username);

    Err(Error::LoginFailed)
}
