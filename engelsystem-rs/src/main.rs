pub mod error;
mod templates;

use std::net::Ipv4Addr;

use actix_files::Files;
use actix_web::{get, web::{Data, Html}, App, HttpServer, Responder};
use askama::Template;
use engelsystem_rs_db::{permission::get_perm_count, role::get_role_count, user::get_user_count, DatabaseConnection};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub use error::*;

#[get("/")]
async fn landing_page(db: Data<DatabaseConnection>) -> Result<impl Responder> {
    let user_count = get_user_count(&db).await?;
    let role_count = get_role_count(&db).await?;
    let perm_count = get_perm_count(&db).await?;
    let rendered = templates::Index {
        org: "Real Org",
        rows: Vec::from([
            ("Benutzer", user_count),
            ("Rollen", role_count),
            ("Berechtigungen", perm_count),
        ])
    }
    .render()?;

    Ok(Html::new(rendered))
}

#[actix_web::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let db = engelsystem_rs_db::connect_and_migrate("sqlite://meow.sqlite?mode=rwc").await?;
    let shared_db = Data::new(db);
    
    HttpServer::new(move || {
        App::new()
            .app_data(shared_db.clone())
            .service(landing_page)
            .service(Files::new("/static", "assets"))
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await?;

    Ok(())
}

