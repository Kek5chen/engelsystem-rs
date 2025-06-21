use actix_web::{HttpResponse, Responder, get, http::header, web::Data};
use snafu::ResultExt;

use crate::{
    generated::BackendErr,
    session::{PublicSession, RequestSessionExt}, utils::response_ext::ActixResponseExt,
};

#[get("/logout")]
pub async fn request_logout(
    client: Data<reqwest::Client>,
    session: PublicSession,
) -> crate::Result<impl Responder> {
    let Some(session) = session.upgrade() else {
        return Ok(HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/"))
            .finish());
    };

    const LOGOUT_URL: &str = "http://127.0.0.1:8081/logout";
    client
        .get(LOGOUT_URL)
        .add_session(&session)
        .send()
        .await
        .context(BackendErr)?;

    Ok(HttpResponse::SeeOther()
        .expire_session()
        .append_header((header::LOCATION, "/"))
        .finish())
}
