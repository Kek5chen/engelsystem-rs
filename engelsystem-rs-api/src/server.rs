use std::{env, net::Ipv4Addr, process::exit};

use crate::error::generated::*;
use crate::routes::*;
use crate::session_db::DbSessionStore;
use actix_session::SessionMiddleware;
use actix_web::{App, HttpServer, cookie::Key, web::Data};
use apistos::{
    SwaggerUIConfig,
    app::{BuildConfig, OpenApiWrapper},
    info::Info,
    spec::Spec,
    web::{ServiceConfig, get, post, put, resource, scope},
};
use engelsystem_rs_db::connect_and_migrate;
use snafu::ResultExt;
use tracing::warn;

const DEFAULT_DATABASE_URL: &str = "sqlite://meow.sqlite?mode=rwc";
const DEFAULT_PORT: u16 = 8081;
const SESSION_COOKIE_NAME: &str = "session-id";
const DUMMY_SECRET_KEY: &[u8; 64] =
    b"7E8CDED394A2BC2EB3547B16F6C4259DFF4B8218BDA5DF224E27CE44AC999999";

#[derive(Debug)]
struct ServerConfig {
    database_url: String,
    secret_key: Vec<u8>,
    port: u16,
}

impl ServerConfig {
    fn from_env() -> Self {
        let database_url = Self::get_database_url();
        let secret_key = Self::get_secret_key();
        let port = Self::get_port();

        Self {
            database_url,
            secret_key,
            port,
        }
    }

    fn get_database_url() -> String {
        env::var("DATABASE_URL").unwrap_or_else(|_| {
            warn!("No DATABASE_URL set. Using an sqlite file in the current directory.");
            DEFAULT_DATABASE_URL.to_string()
        })
    }

    fn get_secret_key() -> Vec<u8> {
        env::var("SECRET")
            .map(|s| s.as_bytes().to_owned())
            .unwrap_or_else(|_| Self::handle_missing_secret())
    }

    fn handle_missing_secret() -> Vec<u8> {
        if cfg!(debug_assertions) {
            warn!(
                "No SECRET set. Using default secret. This is unsafe in production and only for debug mode."
            );
            DUMMY_SECRET_KEY.to_vec()
        } else {
            warn!("No SECRET set. This is a release build so we will not generate one.");
            exit(1);
        }
    }

    fn get_port() -> u16 {
        env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(DEFAULT_PORT)
    }
}

fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(resource("/register").route(post().to(request_register)))
        .service(resource("/login").route(post().to(request_login)))
        .service(resource("/logout").route(get().to(request_logout)))
        .service(resource("/users").route(get().to(user_list)))
        .service(resource("/users/{user_id}").route(get().to(view_user)))
        .service(resource("/me").route(get().to(view_me)))
        .service(resource("/stats/user_count").route(get().to(user_count)))
        .service(resource("/settings").route(post().to(update_settings)))
        .service(
            scope("/shifts")
                .service(resource("/").route(put().to(shift_add)))
                .service(resource("/me").route(get().to(shifts_self))),
        );
}

async fn initialize_database(database_url: &str) -> crate::Result<engelsystem_rs_db::Database> {
    connect_and_migrate(database_url).await.context(DatabaseErr)
}

fn api_spec() -> Spec {
    Spec {
        info: Info {
            title: "Engelsystem RS API".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            ..Default::default()
        },
        ..Default::default()
    }
}

async fn start_server(
    config: ServerConfig,
    shared_db: Data<engelsystem_rs_db::Database>,
) -> crate::Result<()> {
    HttpServer::new(move || {
        App::new()
            .document(api_spec())
            .wrap(
                SessionMiddleware::builder(
                    DbSessionStore::new(shared_db.clone()),
                    Key::from(&config.secret_key),
                )
                .cookie_name(SESSION_COOKIE_NAME.to_string())
                .build(),
            )
            .app_data(shared_db.clone())
            .configure(configure_routes)
            .build_with(
                "/openapi.json",
                BuildConfig::default().with(SwaggerUIConfig::new(&"swagger")),
            )
    })
    .bind((Ipv4Addr::UNSPECIFIED, config.port))
    .context(WebserverErr)?
    .run()
    .await
    .context(WebserverErr)
}

pub async fn run_server() -> crate::Result<()> {
    let config = ServerConfig::from_env();

    let db = initialize_database(&config.database_url).await?;
    let shared_db = Data::new(db);

    start_server(config, shared_db).await
}
