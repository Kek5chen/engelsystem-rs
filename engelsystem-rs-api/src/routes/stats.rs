use actix_web::web::{Data, Json};
use apistos::{ApiComponent, api_operation};
use engelsystem_rs_db::{
    DatabaseConnection,
    user::{get_admin_count, get_guest_count, get_user_count},
};
use schemars::JsonSchema;
use serde::Serialize;
use snafu::ResultExt;

use crate::generated::DatabaseErr;

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct UserCountStats {
    total: u64,
    admin: u64,
    guest: u64,
}

async fn fetch_user_count_stats(db: &DatabaseConnection) -> crate::Result<UserCountStats> {
    Ok(UserCountStats {
        total: get_user_count(db).await.context(DatabaseErr)?,
        admin: get_admin_count(db).await.context(DatabaseErr)?,
        guest: get_guest_count(db).await.context(DatabaseErr)?,
    })
}

#[api_operation(
    tag = "statistics",
    summary = "Get global user count statistics"
)]
pub async fn user_count(db: Data<DatabaseConnection>) -> crate::Result<Json<UserCountStats>> {
    Ok(Json(fetch_user_count_stats(&db).await?))
}
