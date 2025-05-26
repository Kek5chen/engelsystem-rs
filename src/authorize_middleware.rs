use std::future::{ready, Ready};

use actix_session::SessionExt;
use actix_web::FromRequest;
use engelsystem_rs_db::role::RoleType;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use uuid::Uuid;

use crate::{generated::SessionDeserializeErr, Error};

#[derive(Serialize, Deserialize)]
pub struct BasicUserAuth {
    user_id: Uuid,
    role_id: RoleType,
}

impl FromRequest for BasicUserAuth {
    type Error = Error;

    type Future = Ready<crate::Result<BasicUserAuth>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        ready(Self::_from_request(req))
    }
}

impl BasicUserAuth {
    fn _from_request(req: &actix_web::HttpRequest) -> crate::Result<Self> {
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

        Ok(BasicUserAuth { user_id, role_id })
    }
}
