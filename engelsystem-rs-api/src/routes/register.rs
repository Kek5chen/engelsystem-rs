use actix_web::web::{Data, Json};
use apistos::{ApiComponent, actix::NoContent, api_operation};
use engelsystem_rs_db::{
    DatabaseConnection,
    user::{self},
};
use schemars::JsonSchema;
use serde::Deserialize;
use snafu::ResultExt;
use tracing::info;
use validator::Validate;
use zeroize::Zeroizing;

use crate::utils::{schema_impls::ZeroizingDef, validation::*};
use crate::{Error, generated::DatabaseErr};

// TODO: Validate better
#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct RegistrationData {
    #[validate(custom(function = "validate_username"))]
    username: String,
    #[validate(email(message = "Die Email ist nicht korrekt"))]
    email: String,
    #[validate(custom(function = "validate_password"))]
    #[serde(with = "ZeroizingDef::<String>")]
    password: Zeroizing<String>,
    #[serde(rename = "tc_check")]
    _tc_check: String,
}

#[api_operation(summary = "Request to register a new user account")]
pub async fn request_register(
    Json(data): Json<RegistrationData>,
    db: Data<DatabaseConnection>,
) -> crate::Result<NoContent> {
    let errors = data.validate().err().map(|e| {
        e.field_errors()
            .into_iter()
            .map(
                |(key, errs)| match errs.first().and_then(|e| e.message.as_ref()) {
                    Some(msg) => msg.clone(),
                    None => key,
                },
            )
            .collect::<Vec<_>>()
    });

    if errors.is_some() {
        return Err(Error::RegisterValidationFailed);
    }

    info!("User {:?} registered", data.username);

    match user::add_guest(data.username, data.email, &data.password, &db).await {
        Ok(_) => Ok(NoContent),
        Err(engelsystem_rs_db::Error::UserExists) => Err(Error::UserExists),
        Err(e) => Err(e).context(DatabaseErr),
    }
}
