pub mod error;

use std::net::Ipv4Addr;

use actix_files::Files;
use actix_web::{get, web::{Data, Html}, App, HttpServer, Responder};
use engelsystem_rs_db::{permission::get_perm_count, role::get_role_count, user::get_user_count, DatabaseConnection};
use serde_json::json;
use tera::{Context, Tera};
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub use error::*;

#[actix_web::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let templates = Tera::new("templates/*")?;
    for template in templates.get_template_names() {
        debug!("loaded: {template}");
    }
    let shared_templates = Data::new(templates);

    let db = engelsystem_rs_db::connect_and_migrate("sqlite://meow.sqlite?mode=rwc").await?;
    let shared_db = Data::new(db);
    
    HttpServer::new(move || {
        App::new()
            .app_data(shared_db.clone())
            .app_data(shared_templates.clone())
            .service(landing_page)
            .service(register)
            .service(login)
            .service(Files::new("/static", "assets"))
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await?;

    Ok(())
}

#[get("/register")]
async fn register(templates: Data<Tera>) -> Result<impl Responder> {
    let context = Context::from_serialize(json!({
        "org": "Real Org",
    })).unwrap();
    let rendered = templates.render("register.html", &context).unwrap();
    Ok(Html::new(rendered))
}

#[get("/login")]
async fn login(templates: Data<Tera>) -> Result<impl Responder> {
    let context = Context::from_serialize(json!({
        "org": "Real Org",
    })).unwrap();
    let rendered = templates.render("login.html", &context).unwrap();
    Ok(Html::new(rendered))
}

#[get("/")]
async fn landing_page(db: Data<DatabaseConnection>, templates: Data<Tera>) -> Result<impl Responder> {
    let user_count = get_user_count(&db).await?;
    let role_count = get_role_count(&db).await?;
    let perm_count = get_perm_count(&db).await?;

    let context = Context::from_serialize(json!({
        "org": "Real Org",
        "rows": {
            "Benutzer": user_count,
            "Rollen": role_count,
            "Berechtigungen": perm_count
        }
    }))?;
    let rendered = templates.render("landing.html", &context).unwrap();

    Ok(Html::new(rendered))
}

