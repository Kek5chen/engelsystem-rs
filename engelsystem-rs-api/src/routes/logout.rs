use actix_session::Session;
use actix_web::{HttpResponse, Responder, post};

#[post("/logout")]
pub async fn request_logout(session: Session) -> crate::Result<impl Responder> {
    session.clear();

    Ok(HttpResponse::Ok())
}
