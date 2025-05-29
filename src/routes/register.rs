use actix_web::{get, http::header::{self, ContentType}, post, web::{self, Data, Html}, HttpResponse, Responder};
use engelsystem_rs_db::{user::{self}, DatabaseConnection};
use serde::Deserialize;
use snafu::{IntoError, ResultExt};
use tera::{Context, Tera};
use validator::Validate;
use zeroize::Zeroizing;
use crate::utils::validation::*;

use crate::generated::DatabaseErr;

// TODO: Validate better
#[derive(Debug, Deserialize, Validate)]
struct RegistrationData {
    #[validate(custom(function = "validate_username"))]
    username: String,
    #[validate(email(message = "Die Email ist nicht korrekt"))]
    email: String,
    #[validate(custom(function = "validate_password"))]
    password: Zeroizing<String>,
    #[serde(rename = "tc_check")]
    _tc_check: String,
}

#[get("/register")]
async fn register_page(templates: Data<Tera>) -> crate::Result<impl Responder> {
    let mut context = Context::new();
    context.insert("org", "Real Org");

    let rendered = templates.render("register.html", &context)
        .map_err(|e| {
            tracing::error!("Template error: {e}");
            crate::error::generated::TemplateErr.into_error(e)
        })?;

    Ok(Html::new(rendered))
}

#[post("/register")]
async fn request_register(
    web::Form(data): web::Form<RegistrationData>,
    templates: Data<Tera>,
    db: Data<DatabaseConnection>,
) -> crate::Result<impl Responder> {
    let mut context = Context::new();
    context.insert("org", "Real Org");

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

    context.insert("errors", &errors);

    if errors.is_some() {
        let rendered = templates.render("register.html", &context)
            .map_err(|e| {
                tracing::error!("Template error: {e}");
                crate::error::generated::TemplateErr.into_error(e)
            })?;

        return Ok(HttpResponse::BadRequest()
            .content_type(ContentType::html())
            .body(rendered));
    }

    user::add_guest(data.username, data.email, &data.password, &db)
        .await
        .context(DatabaseErr)?;

    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/login?created=true"))
        .finish())
}
