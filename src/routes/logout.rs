use actix_session::Session;
use actix_web::{get, http::header, HttpResponse, Responder};

#[get("/logout")]
pub async fn request_logout(
    session: Session,
) -> crate::Result<impl Responder> {
    session.clear();

    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/login"))
        .finish())
}

