use actix_web::{get, web::Data, HttpResponse, Responder};
use engelsystem_rs_db::{user::{get_admin_count, get_guest_count, get_user_count}, DatabaseConnection};
use serde_json::json;
use snafu::ResultExt;

use crate::generated::DatabaseErr;

#[get("/stats/user_count")]
pub async fn user_count(
    db: Data<DatabaseConnection>,
) -> crate::Result<impl Responder> {
    let total_count = get_user_count(&db).await.context(DatabaseErr)?;
    let admin_count = get_admin_count(&db).await.context(DatabaseErr)?;
    let guest_count = get_guest_count(&db).await.context(DatabaseErr)?;

    Ok(HttpResponse::Ok()
        .json(
            json!({
                "total": total_count,
                "admin": admin_count,
                "guest": guest_count,
            })
        )
    )
}

