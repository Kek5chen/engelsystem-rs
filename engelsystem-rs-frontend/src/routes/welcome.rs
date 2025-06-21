use actix_web::{
    get,
    web::Data,
    HttpResponse, Responder,
};
use engelsystem_rs_db::UserView;
use reqwest::header;
use tera::Tera;

use crate::{
    render_template, session::Session, utils::response_ext::ActixResponseExt,
};

#[get("/welcome")]
async fn welcome_page(
    templates: Data<Tera>,
    client: Data<reqwest::Client>,
    session: Session,
) -> crate::Result<impl Responder> {
    const USER_URL: &str = "http://127.0.0.1:8081/me";
    let user: UserView = client
        .get(USER_URL)
        .header(header::COOKIE, session.cookie())
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;


    Ok(HttpResponse::Ok()
        .html(render_template!(&templates, "welcome.html", session, [ "user" => &user ])?))
}
