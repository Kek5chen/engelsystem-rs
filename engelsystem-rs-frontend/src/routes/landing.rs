use actix_web::{
    Responder, get,
    web::{Data, Html},
};
use serde::Deserialize;
use serde_json::json;
use snafu::{IntoError, ResultExt};
use tera::Tera;

use crate::{generated::BackendErr, render_template};

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

    let rendered = render_template!(&templates, "landing.html", [
        "rows" => &{
            json!([
                { "Benutzer": counts.total },
                { "Gäste": counts.guest },
                { "Admins": counts.admin }
            ])
        }
    ])?;

    Ok(Html::new(rendered))
}
