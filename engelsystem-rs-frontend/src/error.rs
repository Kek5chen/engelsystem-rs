use actix_web::ResponseError;
use snafu::Snafu;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(Err)),module(generated),visibility(pub(crate)))]
pub enum Error {
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

    #[snafu(display("The backend returned an unexpected response: {source}"))]
    Backend {
        source: reqwest::Error,
    },

    #[snafu(display("The backend did not return the expected cookie: {name}"))]
    BackendCookieInvalid {
        name: String,
    },

    #[snafu(display("You are not authorized to access this resource"))]
    Unauthorized,
}

impl ResponseError for Error {}
