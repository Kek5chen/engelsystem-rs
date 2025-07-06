use actix_web::web::{Data, Json, Query};
use apistos::{ApiComponent, api_operation};
use chrono::{DateTime, Utc};
use engelsystem_rs_db::{
    ActiveShift, Database, Shift,
    shift::{add_shift, get_shifts_by_user},
    user::{get_angel_type_id_by_name, get_user_id_by_name},
};
use schemars::JsonSchema;
use serde::Deserialize;
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{
    authorize_middleware::{BasicAdminAuth, BasicGuestAuth, BasicUser},
    generated::{AngelTypeNotFoundErr, DatabaseErr, UserNotFoundErr},
};

fn b_true() -> bool {
    true
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct ShiftFilter {
    limit: Option<u32>,
    #[serde(default = "b_true")]
    include_expired: bool,
    #[serde(default = "b_true")]
    include_started: bool,
}

#[api_operation(
    tag = "shift",
    summary = "Get all shifts you are helping out in with optional filters",
    security_scope(name = "session_id", scope = "user",)
)]
pub async fn shifts_self(
    db: Data<Database>,
    user: BasicUser<BasicGuestAuth>,
    Query(filters): Query<ShiftFilter>,
) -> crate::Result<Json<Vec<Shift>>> {
    let shifts = get_shifts_by_user(user.uid, filters.limit, filters.include_expired, filters.include_started, &db)
        .await
        .context(DatabaseErr)?;

    Ok(Json(shifts))
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct NewShift {
    pub managed_by: Option<String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub name: String,
    pub description: Option<String>,
    pub angels_needed: u32,
    pub angel_type: Option<String>,
}

impl NewShift {
    pub async fn prepare(self, created_by: Uuid, db: &Database) -> crate::Result<ActiveShift> {
        use engelsystem_rs_db::ActiveValue::*;

        let managed_by = match self.managed_by {
            Some(name) => Some(
                get_user_id_by_name(&name, db)
                    .await
                    .context(DatabaseErr)?
                    .context(UserNotFoundErr { name })?,
            ),
            None => None,
        };

        let angel_type = match self.angel_type {
            Some(angel_type) => {
                Some(
                    get_angel_type_id_by_name(&angel_type, db)
                        .await
                        .context(DatabaseErr)?
                        .context(AngelTypeNotFoundErr {
                            name: angel_type,
                        })?
                )
            }
            None => None
        };

        Ok(ActiveShift {
            id: NotSet,
            created_at: NotSet,
            created_by: Set(created_by),
            managed_by: Set(managed_by),
            starts_at: Set(self.starts_at),
            ends_at: Set(self.ends_at),
            name: Set(self.name),
            description: Set(self.description),
            angels_needed: Set(self.angels_needed),
            angel_type_id: Set(angel_type),
        })
    }
}

#[api_operation(
    tag = "shift",
    summary = "Add a shift",
    security_scope(name = "session-id", scope = "admin",)
)]
pub async fn shift_add(
    Json(shift): Json<NewShift>,
    db: Data<Database>,
    user: BasicUser<BasicAdminAuth>,
) -> crate::Result<Json<Shift>> {
    let shifts = add_shift(shift.prepare(user.uid, &db).await?, &db)
        .await
        .context(DatabaseErr)?;

    Ok(Json(shifts))
}

