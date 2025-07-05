use actix_session::SessionInsertError;
use actix_web::{ResponseError, http::StatusCode};
use snafu::Snafu;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(Err)), module(generated), visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Database Error: {source}"))]
    Database { source: engelsystem_rs_db::Error },

    #[snafu(display("Webserver Error: {source}"))]
    Webserver { source: std::io::Error },

    #[snafu(transparent)]
    SessionInsert { source: SessionInsertError },

    #[snafu(display("You are not authorized to access this resource"))]
    SessionUnauthorized,

    #[snafu(display("You are not logged in"))]
    SessionUnauthenticated,

    #[snafu(display("Deserialize Session Error: {source}"))]
    SessionDeserialize {
        source: actix_session::SessionGetError,
    },

    #[snafu(display("The given Uid ({uid}) was not valid"))]
    InvalidUid { uid: String },

    #[snafu(display("Es existiert bereits ein Benutzer mit dieser Email"))]
    UserExists,

    #[snafu(display("Es konnte keine Nutzer mit dem Name {name:?} gefunden werden"))]
    UserNotFound { name: String },

    #[snafu(display("Der Engeltyp {name:?} existiert nicht"))]
    AngelTypeNotFound { name: String },
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::SessionUnauthorized => StatusCode::FORBIDDEN,
            Error::SessionUnauthenticated => StatusCode::UNAUTHORIZED,
            Error::InvalidUid { .. } => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
