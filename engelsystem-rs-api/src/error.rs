use actix_session::SessionInsertError;
use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use apistos::ApiErrorComponent;
use snafu::Snafu;
use tracing::error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu, ApiErrorComponent)]
#[openapi_error(
    status(code = 401),
    status(code = 403),
    status(code = 404),
    status(code = 500)
)]
#[snafu(context(suffix(Err)), module(generated), visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Database Error: {source}"))]
    Database {
        source: engelsystem_rs_db::Error,
    },

    #[snafu(display("Webserver Error: {source}"))]
    Webserver {
        source: std::io::Error,
    },

    #[snafu(transparent)]
    SessionInsert {
        source: SessionInsertError,
    },

    #[snafu(display("You are not authorized to access this resource"))]
    SessionUnauthorized,

    #[snafu(display("You are not logged in"))]
    SessionUnauthenticated,

    #[snafu(display("Deserialize Session Error: {source}"))]
    SessionDeserialize {
        source: actix_session::SessionGetError,
    },

    #[snafu(display("The given Uid ({uid}) was not valid"))]
    InvalidUid {
        uid: String,
    },

    LoginFailed,

    RegisterValidationFailed,

    #[snafu(display("Es existiert bereits ein Benutzer mit dieser Email"))]
    UserExists,

    #[snafu(display("Es konnte keine Nutzer mit dem Name {name:?} gefunden werden"))]
    UserNotFound {
        name: String,
    },

    #[snafu(display("Es konnte keine Nutzer mit der ID {uid:?} gefunden werden"))]
    UIDNotFound {
        uid: String,
    },

    #[snafu(display("Der Engeltyp {name:?} existiert nicht"))]
    AngelTypeNotFound {
        name: String,
    },

    #[snafu(display("An internal error ocurred"))]
    GenericInternalError,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::RegisterValidationFailed => StatusCode::BAD_REQUEST,
            Error::SessionUnauthenticated | Error::LoginFailed => StatusCode::UNAUTHORIZED,
            Error::SessionUnauthorized => StatusCode::FORBIDDEN,
            Error::InvalidUid { .. } => StatusCode::NOT_FOUND,
            _ => {
                error!("{self:?} || Readable: {self}");
                StatusCode::INTERNAL_SERVER_ERROR
            },
        }
    }
}
