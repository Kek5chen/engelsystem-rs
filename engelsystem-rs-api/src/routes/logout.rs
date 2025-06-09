use actix_session::Session;
use actix_web::{post, HttpResponse, Responder};

#[post("/logout")]
pub async fn request_logout(
    session: Session,
) -> crate::Result<impl Responder> {
    session.clear();

    Ok(HttpResponse::Ok())
}

