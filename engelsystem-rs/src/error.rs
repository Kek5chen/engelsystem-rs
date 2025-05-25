use actix_session::{SessionGetError, SessionInsertError};
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

    #[snafu(display("Templating Context Error: {source}"))]
    Context {
        source: tera::Error,
    },

    #[snafu(display("Templating Error: {source}"))]
    Template {
        source: tera::Error,
    },

    #[snafu(display("Webserver Error: {source}"))]
    Webserver {
        source: std::io::Error,
    },

    #[snafu(transparent)]
    SessionInsert {
        source: SessionInsertError,
    },

    #[snafu(display("You are unauthorized to access this resource."))]
    SessionUnauthenticated,

    #[snafu(display("Data from the session couldn't be deserialized: {source:?}"))]
    SessionDeserialize {
        source: SessionGetError,
    }
}

impl ResponseError for Error {}
