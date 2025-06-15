use std::{env, net::Ipv4Addr, process::exit};

use crate::error::generated::*;
use crate::routes::*;
use crate::session_db::DbSessionStore;
use actix_session::SessionMiddleware;
use actix_web::{cookie::Key, web::Data, App, HttpServer};
use engelsystem_rs_db::{connect_and_migrate, user::{add_guest, get_user_count}};
use rand::{distr::Alphanumeric, Rng};
use snafu::ResultExt;
use tracing::warn;

const DUMMY_SECRET_KEY: &[u8; 64] =
    b"7E8CDED394A2BC2EB3547B16F6C4259DFF4B8218BDA5DF224E27CE44AC999999";

pub async fn run_server() -> crate::Result<()> {
    let url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        warn!("No DATABASE_URL set. Using an sqlite file in the current directory.");
        "sqlite://meow.sqlite?mode=rwc".to_string()
    });
    let secret = env::var("SECRET");
    let secret = secret.map(|s| s.as_bytes().to_owned()).unwrap_or_else(|_| {
        if cfg!(debug_assertions) {
            warn!("No SECRET set. Implying a default secret. This is majorly unsafe in production and is only to be used in debug mode.");
            DUMMY_SECRET_KEY.to_vec()
        } else {
            warn!("No SECRET set. This is a release build so we will not generate one.");
            exit(1);
        }
    });

    let db = connect_and_migrate(&url)
        .await
        .context(DatabaseErr)?;

    let shared_db = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(
                    DbSessionStore::new(shared_db.clone()),
                    Key::from(&secret),
                )
                .cookie_name("session-id".to_string())
                .build(),
            )
            .app_data(shared_db.clone())
            .service(request_register)
            .service(request_login)
            .service(request_logout)
            .service(user_list)
            .service(view_user)
            .service(view_me)
            .service(user_count)
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8081))
    .context(WebserverErr)?
    .run()
    .await
    .context(WebserverErr)?;

    Ok(())
}
