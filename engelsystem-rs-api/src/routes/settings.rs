use actix_web::{
    Either,
    web::{Data, Json},
};
use apistos::{
    ApiComponent,
    actix::{AcceptedJson, NoContent},
    api_operation,
};
use engelsystem_rs_db::ActiveValue::*;
use engelsystem_rs_db::{ActiveUser, DatabaseConnection, user::update_user};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::{
    authorize_middleware::{BasicGuestAuth, BasicUser},
    generated::DatabaseErr,
};

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SettingsUpdateRequest {
    username: String,
    email: String,
    password: Option<String>,
    confirm_password: Option<String>,
}

#[api_operation(
    tag = "account",
    summary = "Update user settings",
    security_scope(name = "session-id")
)]
pub async fn update_settings(
    db: Data<DatabaseConnection>,
    session: BasicUser<BasicGuestAuth>,
    Json(new): Json<SettingsUpdateRequest>,
) -> crate::Result<Either<AcceptedJson<()>, NoContent>> {
    let changed = ActiveUser {
        id: NotSet,
        created_at: NotSet,
        member_id: NotSet,
        username: Set(new.username),
        email: Set(new.email),
        password_hash: NotSet,
        role_id: NotSet,
        points: NotSet,
    };

    if update_user(session.uid, changed, &db)
        .await
        .context(DatabaseErr)?
        .is_some()
    {
        Ok(Either::Left(AcceptedJson(())))
    } else {
        Ok(Either::Right(NoContent))
    }
}
