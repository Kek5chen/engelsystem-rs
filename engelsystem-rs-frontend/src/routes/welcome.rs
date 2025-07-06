use std::future;

use actix_web::{HttpResponse, Responder, get, web::Data};
use engelsystem_rs_db::{Shift, UserView};
use tera::Tera;

use crate::{render_template, session::{RequestSessionExt, Session}, utils::response_ext::ActixResponseExt};

#[get("/welcome")]
async fn welcome_page(
    templates: Data<Tera>,
    client: Data<reqwest::Client>,
    session: Session,
) -> crate::Result<impl Responder> {
    const USER_URL: &str = "http://127.0.0.1:8081/me";
    let user: UserView = client
        .get(USER_URL)
        .add_session(&session)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    const SHIFT_URL: &str = "http://127.0.0.1:8081/shifts/me";
    let future_shifts: Vec<Shift> = client.get(SHIFT_URL)
        .query(&[("limit", "1"), ("include_expired", "false"), ("include_started", "false")])
        .add_session(&session)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let next_shift = future_shifts.first();

    Ok(HttpResponse::Ok()
        .html(render_template!(&templates, "welcome.html", session, [ "user" => &user, "next_shift" => &next_shift])?))
}
