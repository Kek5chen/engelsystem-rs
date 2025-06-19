use std::{future::ready, marker::PhantomData, pin::Pin};

use actix_web::{dev::Payload, FromRequest, HttpRequest};

use crate::{templates::BaseData, Error};

pub struct Public;
pub struct EnsurePrivate;

pub type PublicSession = Session<Public>;

pub struct Session<A = EnsurePrivate> {
    session_id: Option<String>,

    _accessibility: PhantomData<A>,
}

impl<A> Session<A> {
    fn new(session_id: Option<String>) -> Self {
        Session {
            session_id,

            _accessibility: PhantomData,
        }
    }

    pub fn base_data<'a>(&self, org: &'a str) -> BaseData<'a> {
        BaseData::new(org, self.session_id.is_some())
    }
}

impl FromRequest for Session<Public> {
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        Box::pin(ready(Ok(req.into_optional_session())))
    }
}

impl FromRequest for Session<EnsurePrivate> {
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        Box::pin(ready(req.into_authenticated_session()))
    }
}


pub trait IntoSession {
    fn into_authenticated_session(self) -> crate::Result<Session<EnsurePrivate>>;
    fn into_optional_session(self) -> Session<Public>;
}

impl IntoSession for &HttpRequest {
    fn into_authenticated_session(self) -> crate::Result<Session<EnsurePrivate>> {
        match self.cookie("session-id") {
            Some(cookie) => Ok(Session::new(Some(cookie.value().to_string()))),
            None => Err(Error::Unauthorized),
        }
    }

    fn into_optional_session(self) -> Session<Public> {
        match self.cookie("session-id") {
            Some(cookie) => Session::new(Some(cookie.value().to_string())),
            None => Session::new(None),
        }
    }
}

