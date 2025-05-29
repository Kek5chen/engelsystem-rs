use actix_session::Session;
use actix_web::{get, http::header::{self, ContentType}, post, web::{self, Data, Html}, HttpResponse, Responder};
use engelsystem_rs_db::{user::verify_user, DatabaseConnection};
use serde::Deserialize;
use snafu::IntoError;
use tera::{Context, Tera};
use tracing::info;
use validator::Validate;
use zeroize::Zeroizing;
use crate::utils::validation::*;

#[derive(Deserialize)]
struct LoginPageData {
    created: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
struct LoginData {
    #[validate(custom(function = "validate_username"))]
    username: String,
    #[validate(custom(function = "validate_password"))]
    password: Zeroizing<String>,
}

#[get("/login")]
pub async fn login_page(
    web::Query(data): web::Query<LoginPageData>,
    templates: Data<Tera>,
) -> crate::Result<impl Responder> {
    let mut context = Context::new();
    context.insert("org", "Real Org");
    context.insert("created", &data.created.unwrap_or(false));

    let rendered = templates.render("login.html", &context)
        .map_err(|e| {
            tracing::error!("Template error: {e}");
            crate::error::generated::TemplateErr.into_error(e)
        })?;
    Ok(Html::new(rendered))
}

#[post("/login")]
pub async fn request_login(
    web::Form(data): web::Form<LoginData>,
    templates: Data<Tera>,
    db: Data<DatabaseConnection>,
    session: Session,
) -> crate::Result<impl Responder> {
    let user = verify_user(&data.username, &data.password, &db).await;

    if let Some(user) = user {
        session.clear();
        session.insert("user_id", user.id)?;
        session.insert("role_id", user.role_id)?;

        info!("User {:?} logged in successfully", user.username);

        return Ok(HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/welcome"))
            .finish());
    }

    info!("User {:?} failed to login.", data.username);

    let mut context = Context::new();
    context.insert("org", "Real Org");
    context.insert("error", &true);

    let rendered = templates.render("login.html", &context)
        .map_err(|e| {
            tracing::error!("Template error: {e}");
            crate::error::generated::TemplateErr.into_error(e)
        })?;

    Ok(HttpResponse::Unauthorized()
        .content_type(ContentType::html())
        .body(rendered))
}
