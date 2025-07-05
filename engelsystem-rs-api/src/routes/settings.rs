use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};
use engelsystem_rs_db::ActiveValue::*;
use engelsystem_rs_db::{ActiveUser, DatabaseConnection, user::update_user};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::{
    authorize_middleware::{BasicGuestAuth, BasicUser},
    generated::DatabaseErr,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsUpdateRequest {
    username: String,
    email: String,
    password: Option<String>,
    confirm_password: Option<String>,
}

#[post("/settings")]
pub async fn update_settings(
    db: Data<DatabaseConnection>,
    session: BasicUser<BasicGuestAuth>,
    Json(new): Json<SettingsUpdateRequest>,
) -> crate::Result<impl Responder> {
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

    dbg!(&changed);

    if update_user(session.uid, changed, &db)
        .await
        .context(DatabaseErr)?
        .is_some()
    {
        Ok(HttpResponse::Ok())
    } else {
        dbg!("No change");
        Ok(HttpResponse::NoContent())
    }
}
