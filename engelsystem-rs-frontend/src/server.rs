use std::net::Ipv4Addr;

use crate::error::generated::*;
use crate::routes::*;
use actix_files::Files;
use actix_web::{App, HttpServer, web::Data};
use snafu::ResultExt;
use tera::Tera;
use tracing::debug;

pub async fn run_server() -> crate::Result<()> {
    let templates = match Tera::new("templates/*").context(TemplateErr) {
        Ok(tmpls) => tmpls,
        Err(e) => {
            tracing::error!("Couldn't load templates: {e}");
            std::process::exit(1);
        }
    };

    for template in templates.get_template_names() {
        debug!("loaded: {template}");
    }
    let shared_templates = Data::new(templates);
    let shared_client = Data::new(reqwest::Client::new());

    HttpServer::new(move || {
        App::new()
            .app_data(shared_client.clone())
            .app_data(shared_templates.clone())
            .service(landing_page)
            .service(register_page)
            .service(login_page)
            .service(request_register)
            .service(request_login)
            .service(request_logout)
            .service(welcome_page)
            .service(user_list)
            .service(view_user)
            .service(Files::new("/static", "assets"))
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))
    .context(WebserverErr)?
    .run()
    .await
    .context(WebserverErr)?;

    Ok(())
}
