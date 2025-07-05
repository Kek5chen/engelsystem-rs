use std::{marker::PhantomData, pin::Pin};

use actix_session::SessionExt;
use actix_web::{FromRequest, HttpRequest, dev::Payload};
use engelsystem_rs_db::role::RoleType;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use uuid::Uuid;

use crate::{Error, generated::SessionDeserializeErr};

trait BasicResolveSessionImpl {
    fn basic_resolve_session<A: BasicAuthTrait>(
        req: &actix_web::HttpRequest,
    ) -> crate::Result<BasicUser<A>> {
        let session = req.get_session();
        let user_id: Uuid = session
            .get("user_id")
            .context(SessionDeserializeErr)?
            .ok_or(Error::SessionUnauthenticated)?;
        // TODO: This should be fairly safe. Though this should probably fail properly usually
        let role_id: RoleType = RoleType::from_or_default(
            session
                .get::<u32>("role_id")
                .context(SessionDeserializeErr)?
                .ok_or(Error::SessionUnauthenticated)?,
        );

        Ok(BasicUser::new(user_id, role_id))
    }
}

impl<T: BasicAuthTrait> BasicResolveSessionImpl for T {}

pub trait BasicAuthTrait: Sized + 'static {
    fn authenticate(
        user: BasicUser<Self>,
        req: actix_web::HttpRequest,
    ) -> impl Future<Output = crate::Result<BasicUser<Self>>>;
}

#[derive(Serialize, Deserialize)]
pub struct BasicUser<AuthType: BasicAuthTrait> {
    pub uid: Uuid,
    pub role: RoleType,

    _auth_type: PhantomData<AuthType>,
}

impl<AuthType: BasicAuthTrait> BasicUser<AuthType> {
    pub fn new(uid: Uuid, role: RoleType) -> BasicUser<AuthType> {
        BasicUser {
            uid,
            role,
            _auth_type: PhantomData,
        }
    }
}

impl<A: BasicAuthTrait> FromRequest for BasicUser<A> {
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = crate::Result<Self>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let user = A::basic_resolve_session(&req)?;
            A::authenticate(user, req).await
        })
    }
}

pub struct BasicAdminAuth(());
impl BasicAuthTrait for BasicAdminAuth {
    async fn authenticate(
        user: BasicUser<Self>,
        _req: HttpRequest,
    ) -> crate::Result<BasicUser<Self>> {
        if user.role.is_bypass() {
            Ok(user)
        } else {
            Err(Error::SessionUnauthorized)
        }
    }
}

pub struct BasicUserAuth(());
impl BasicAuthTrait for BasicUserAuth {
    async fn authenticate(
        user: BasicUser<Self>,
        _req: HttpRequest,
    ) -> crate::Result<BasicUser<Self>> {
        if user.role.is_bypass() || user.role == RoleType::User {
            Ok(user)
        } else {
            Err(Error::SessionUnauthorized)
        }
    }
}

pub struct BasicGuestAuth(());
impl BasicAuthTrait for BasicGuestAuth {
    async fn authenticate(
        user: BasicUser<Self>,
        _req: HttpRequest,
    ) -> crate::Result<BasicUser<Self>> {
        Ok(user)
    }
}
