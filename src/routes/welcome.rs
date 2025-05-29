use actix_web::{get, web::{Data, Html}, Responder};
use snafu::IntoError;
use tera::{Context, Tera};

use crate::authorize_middleware::{BasicGuestAuth, BasicUser};

#[get("/welcome")]
async fn welcome_page(
    templates: Data<Tera>,
    user: BasicUser<BasicGuestAuth>,
) -> crate::Result<impl Responder> {
    let mut context = Context::new();

    context.insert("org", "Real Org");
    context.insert("user", &user);
    context.insert("logged_in", &true);

    Ok(Html::new(
        templates.render("welcome.html", &context)
            .map_err(|e| {
                tracing::error!("Template error: {e}");
                crate::error::generated::TemplateErr.into_error(e)
            })?
    ))
}

