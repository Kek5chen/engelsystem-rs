use actix_web::{cookie::Cookie, get, http::header, web::Data, HttpRequest, HttpResponse, Responder};
use snafu::ResultExt;

use crate::{generated::BackendErr, Error};

#[get("/logout")]
pub async fn request_logout(
    client: Data<reqwest::Client>,
    req: HttpRequest,
) -> crate::Result<impl Responder> {
    let Some(session_id) = req.cookie("session-id") else {
        return Err(Error::Unauthorized);
    };

    const LOGOUT_URL: &str = "http://127.0.0.1:8081/logout";
    client.get(LOGOUT_URL)
        .header(reqwest::header::COOKIE, format!("session-id={}", session_id.value()))
        .send()
        .await
        .context(BackendErr)?;

    let mut expire_cookie = Cookie::new("session-id", "");
    expire_cookie.make_removal();

    Ok(HttpResponse::SeeOther()
        .cookie(expire_cookie)
        .append_header((header::LOCATION, "/login"))
        .finish())
}

