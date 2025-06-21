use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use reqwest::StatusCode as ReqwestStatusCode;
use snafu::Snafu;
use tracing::error;
use crate::utils::response_ext::ActixResponseExt;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(Err)), module(generated), visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Templating Context Error: {source}"))]
    Context { source: tera::Error },

    #[snafu(display("Templating Error: {source}"))]
    Template { source: tera::Error },

    #[snafu(display("Webserver Error: {source}"))]
    Webserver { source: std::io::Error },

    #[snafu(display("The backend returned an unexpected response: {source}"))]
    Backend { source: reqwest::Error },

    #[snafu(display("The backend did not return the expected cookie: {name}"))]
    BackendCookieInvalid { name: String },

    #[snafu(display("You are not authorized to access this resource"))]
    Unauthorized,

    #[snafu(display("Nopesie"))]
    Forbidden,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        let Some(status) = err.status() else {
            error!("Backend Error: {err:?}");
            return Error::Backend { source: err };
        };

        match status {
            ReqwestStatusCode::FORBIDDEN => Error::Forbidden,
            ReqwestStatusCode::UNAUTHORIZED => Error::Unauthorized,
            _ => {
                error!("Backend Error: {err:?}");
                Error::Backend { source: err }
            }
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let status = self.status_code();
        let mut res = HttpResponse::build(status);

        if status == StatusCode::UNAUTHORIZED {
            res.status(StatusCode::SEE_OTHER);
            res.redirect_to("/")
                .expire_session();
        }

        res.body(BoxBody::new(self.to_string()))
    }
}
