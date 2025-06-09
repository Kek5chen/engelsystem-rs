use actix_session::SessionInsertError;
use actix_web::ResponseError;
use snafu::Snafu;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(Err)),module(generated),visibility(pub(crate)))]
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

    #[snafu(display("Es existiert bereits ein Benutzer mit dieser Email"))]
    UserExists,
}

impl ResponseError for Error {}
