use actix_session::Session;
use apistos::{actix::NoContent, api_operation};

#[api_operation(
    tag = "account",
    summary = "Log out of the current session",
    skip_args = "session"
)]
pub async fn request_logout(session: Session) -> NoContent {
    session.clear();

    NoContent
}
