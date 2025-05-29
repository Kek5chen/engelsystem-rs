use actix_web::{get, web::{Data, Html}, Responder};
use engelsystem_rs_db::{permission::get_perm_count, role::get_role_count, user::get_user_count, DatabaseConnection};
use serde_json::json;
use snafu::{IntoError, ResultExt};
use tera::{Context, Tera};

use crate::generated::{ContextErr, DatabaseErr};

#[get("/")]
pub async fn landing_page(
    db: Data<DatabaseConnection>,
    templates: Data<Tera>,
) -> crate::Result<impl Responder> {
    let user_count = get_user_count(&db).await.context(DatabaseErr)?;
    let role_count = get_role_count(&db).await.context(DatabaseErr)?;
    let perm_count = get_perm_count(&db).await.context(DatabaseErr)?;

    let context = Context::from_value(json!({
        "org": "Real Org",
        "rows": {
            "Benutzer": user_count,
            "Rollen": role_count,
            "Berechtigungen": perm_count
        }
    }))
    .context(ContextErr)?;
    let rendered = templates.render("landing.html", &context)
        .map_err(|e| {
            tracing::error!("Template error: {e}");
            crate::error::generated::TemplateErr.into_error(e)
        })?;


    Ok(Html::new(rendered))
}
