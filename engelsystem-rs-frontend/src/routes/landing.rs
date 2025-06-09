use actix_web::{
    get,
    web::{Data, Html},
    Responder,
};
use serde::Deserialize;
use serde_json::json;
use snafu::{IntoError, ResultExt};
use tera::{Context, Tera};

use crate::generated::{BackendErr, TemplateErr};

#[derive(Deserialize)]
struct UserCountData {
    total: u64,
    admin: u64,
    guest: u64,
}

#[get("/")]
pub async fn landing_page(
    templates: Data<Tera>,
    client: Data<reqwest::Client>,
) -> crate::Result<impl Responder> {
    const COUNT_URL: &str = "http://localhost:8081/stats/user_count";
    let counts: UserCountData = client
        .get(COUNT_URL)
        .send()
        .await
        .context(BackendErr)?
        .json()
        .await
        .context(BackendErr)?;

    let mut context = Context::new();
    context.insert("org", "Real Org");
    context.insert(
        "rows",
        &json!({
            "Benutzer": counts.total,
            "Admins": counts.admin,
            "Gäste": counts.guest

        }),
    );

    let rendered = templates.render("landing.html", &context).map_err(|e| {
        tracing::error!("Template error: {e:?}");
        TemplateErr.into_error(e)
    })?;

    Ok(Html::new(rendered))
}
