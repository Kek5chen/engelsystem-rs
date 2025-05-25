mod authorize_middleware;
pub mod error;
mod session_db;

use std::{borrow::Cow, net::Ipv4Addr};

use actix_files::Files;
use actix_session::{Session, SessionMiddleware};
use actix_web::{
    cookie::Key,
    get,
    http::header::{self, ContentType},
    post,
    web::{self, Data, Html},
    App, HttpResponse, HttpServer, Responder,
};
use authorize_middleware::BasicUserAuth;
use engelsystem_rs_db::{
    permission::get_perm_count,
    role::get_role_count,
    user::{self, get_user_count, verify_user},
    DatabaseConnection,
};
use generated::{ContextErr, DatabaseErr, TemplateErr, WebserverErr};
use serde::Deserialize;
use serde_json::json;
use session_db::DbSessionStore;
use snafu::ResultExt;
use tera::{Context, Tera};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub use error::*;
use validator::{Validate, ValidationError};
use zeroize::Zeroizing;

const DUMMY_SECRET_KEY: &[u8; 64] =
    b"7E8CDED394A2BC2EB3547B16F6C4259DFF4B8218BDA5DF224E27CE44AC999999";

#[actix_web::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let templates = Tera::new("templates/*").context(TemplateErr)?;
    for template in templates.get_template_names() {
        debug!("loaded: {template}");
    }
    let shared_templates = Data::new(templates);

    let db = engelsystem_rs_db::connect_and_migrate("sqlite://meow.sqlite?mode=rwc")
        .await
        .context(DatabaseErr)?;
    let shared_db = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(
                    DbSessionStore::new(shared_db.clone()),
                    Key::from(DUMMY_SECRET_KEY),
                )
                .cookie_name("session-id".to_string())
                .build(),
            )
            .app_data(shared_db.clone())
            .app_data(shared_templates.clone())
            .service(landing_page)
            .service(register)
            .service(login)
            .service(request_register)
            .service(request_login)
            .service(logout)
            .service(welcome)
            .service(Files::new("/static", "assets"))
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))
    .context(WebserverErr)?
    .run()
    .await
    .context(WebserverErr)?;

    Ok(())
}

#[get("/register")]
async fn register(templates: Data<Tera>) -> Result<impl Responder> {
    let context = Context::from_value(json!({
        "org": "Real Org",
    }))
    .unwrap();
    let rendered = templates.render("register.html", &context).unwrap();
    Ok(Html::new(rendered))
}

#[derive(Deserialize)]
struct LoginPageData {
    created: Option<bool>,
}

#[get("/login")]
async fn login(
    web::Query(data): web::Query<LoginPageData>,
    templates: Data<Tera>,
) -> Result<impl Responder> {
    let context = Context::from_value(json!({
        "org": "Real Org",
        "created": data.created.unwrap_or(false),
    }))
    .unwrap();

    let rendered = templates.render("login.html", &context).unwrap();
    Ok(Html::new(rendered))
}

fn validate_password(password: &Zeroizing<String>) -> Result<(), ValidationError> {
    if password.len() < 8 {
        Err(
            ValidationError::new("password_too_short").with_message(Cow::Borrowed(
                "Das Passwort muss mindestens 8 Zeichen beihnalten",
            )),
        )
    } else if password
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && !"!@#%^&_+=;:.,".contains(c))
    {
        Err(
            ValidationError::new("password_invalid_char").with_message(Cow::Borrowed(
                "Das Passwort darf nur A-z, 0-9, oder !@#%^&_+=;:., beinhalten",
            )),
        )
    } else {
        Ok(())
    }
}

fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.len() < 2 {
        Err(
            ValidationError::new("username_too_short").with_message(Cow::Borrowed(
                "Der Benutzername muss mindestens 2 Zeichen beihnalten",
            )),
        )
    } else if username
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && !"_.#".contains(c))
    {
        Err(
            ValidationError::new("username_invalid_char").with_message(Cow::Borrowed(
                "Der Benutzername darf nur A-z, 0-9, oder _.# beinhalten",
            )),
        )
    } else {
        Ok(())
    }
}

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

#[post("/register")]
async fn request_register(
    web::Form(data): web::Form<RegistrationData>,
    templates: Data<Tera>,
    db: Data<DatabaseConnection>,
) -> Result<impl Responder> {
    let mut context = Context::from_value(json!({
        "org": "Real Org",
    }))
    .unwrap();

    let errors = data.validate().err().map(|e| {
        e.field_errors()
            .into_iter()
            .map(
                |(key, errs)| match errs.first().map(|e| e.message.as_ref()).flatten() {
                    Some(msg) => msg.clone(),
                    None => key,
                },
            )
            .collect::<Vec<_>>()
    });

    context.insert("errors", &errors);

    if errors.is_some() {
        let rendered = templates.render("register.html", &context).unwrap();
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

#[derive(Debug, Deserialize, Validate)]
struct LoginData {
    #[validate(custom(function = "validate_username"))]
    username: String,
    #[validate(custom(function = "validate_password"))]
    password: Zeroizing<String>,
}

#[post("/login")]
async fn request_login(
    web::Form(data): web::Form<LoginData>,
    templates: Data<Tera>,
    db: Data<DatabaseConnection>,
    session: Session,
) -> Result<impl Responder> {
    let user = verify_user(&data.username, &data.password, &db).await;

    if let Some(user) = user {
        session.insert("user_id", user.id)?;
        session.insert("role_id", user.role_id)?;

        info!("User {:?} logged in successfully", user.username);

        return Ok(HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/welcome"))
            .finish());
    }

    info!("User {:?} failed to login.", data.username);

    let context = Context::from_value(json!({
        "org": "Real Org",
        "error": true,
    }))
    .unwrap();
    let rendered = templates.render("login.html", &context).unwrap();

    Ok(HttpResponse::Unauthorized()
        .content_type(ContentType::html())
        .body(rendered))
}

#[get("/logout")]
async fn logout(
    session: Session,
) -> Result<impl Responder> {
    session.clear();

    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/login"))
        .finish())
}

#[get("/welcome")]
async fn welcome(
    templates: Data<Tera>,
    user: BasicUserAuth,
) -> Result<impl Responder> {
    let context = Context::from_value(json!({
        "org": "Real Org",
        "user": user,
        "logged_in": true,
    })).unwrap();

    Ok(Html::new(
        templates.render("welcome.html", &context).unwrap(),
    ))
}

#[get("/")]
async fn landing_page(
    db: Data<DatabaseConnection>,
    templates: Data<Tera>,
) -> Result<impl Responder> {
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
    let rendered = templates.render("landing.html", &context).unwrap();

    Ok(Html::new(rendered))
}
