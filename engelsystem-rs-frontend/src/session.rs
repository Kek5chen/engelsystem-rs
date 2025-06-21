use std::{future::ready, marker::PhantomData, pin::Pin};

use actix_web::{FromRequest, HttpRequest, dev::Payload};
use reqwest::RequestBuilder;

use crate::{Error, templates::BaseData};

pub struct Public;
pub struct Authenticated;

pub type PublicSession = Session<Public>;

pub struct Session<A = Authenticated> {
    session_id: Option<String>,

    _accessibility: PhantomData<A>,
}

impl Session<Public> {
    fn new_opt(session_id: Option<String>) -> Self {
        Session {
            session_id,

            _accessibility: PhantomData,
        }
    }

    pub fn upgrade(self) -> Option<Session<Authenticated>> {
        Some(Session::new(self.session_id?))
    }
}

impl<A> Session<A> {
    fn new(session_id: String) -> Self {
        Session {
            session_id: Some(session_id),

            _accessibility: PhantomData,
        }
    }

    pub fn base_data<'a>(&self, org: &'a str) -> BaseData<'a> {
        BaseData::new(org, self.session_id.is_some())
    }

    pub fn exists(&self) -> bool {
        self.session_id.is_some()
    }

    pub fn cookie_opt(&self) -> Option<String> {
        self.session_id
            .as_ref()
            .map(|sid| format!("session-id={sid}"))
    }
}

impl Session<Authenticated> {
    pub fn cookie(&self) -> String {
        format!(
            "session-id={}",
            self.session_id
                .as_ref()
                .expect("EnsurePrivate shouldn't be able to contain no session id")
        )
    }
}

impl FromRequest for Session<Public> {
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        Box::pin(ready(Ok(req.into_optional_session())))
    }
}

impl FromRequest for Session<Authenticated> {
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        Box::pin(ready(req.into_authenticated_session()))
    }
}

pub trait IntoSession {
    fn into_authenticated_session(self) -> crate::Result<Session<Authenticated>>;
    fn into_optional_session(self) -> Session<Public>;
}

impl IntoSession for &HttpRequest {
    fn into_authenticated_session(self) -> crate::Result<Session<Authenticated>> {
        match self.cookie("session-id") {
            Some(cookie) => Ok(Session::new(cookie.value().to_string())),
            None => Err(Error::Unauthorized),
        }
    }

    fn into_optional_session(self) -> Session<Public> {
        Session::new_opt(self.cookie("session-id").map(|c| c.value().to_string()))
    }
}

pub trait RequestSessionExt {
    fn add_session(self, session: &Session<Authenticated>) -> Self;
}

impl RequestSessionExt for RequestBuilder {
    fn add_session(self, session: &Session<Authenticated>) -> Self {
        self.header(reqwest::header::COOKIE, session.cookie())
    }
}

pub trait ResponseCookieExt {
    fn cookie<'a>(&'a self, name: &str) -> Option<reqwest::cookie::Cookie<'a>>;
}

impl ResponseCookieExt for reqwest::Response {
    fn cookie<'a>(&'a self, name: &str) -> Option<reqwest::cookie::Cookie<'a>> {
        self.cookies().find(|c| c.name() == name)
    }
}
