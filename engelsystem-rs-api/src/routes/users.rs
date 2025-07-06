use std::str::FromStr;

use actix_web::web::{self, Data, Json};
use apistos::api_operation;
use engelsystem_rs_db::{
    Database, UserView,
    role::RoleType,
    user::{get_all_user_views, get_user_view_by_id},
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{
    Error,
    authorize_middleware::{BasicAdminAuth, BasicAuthTrait, BasicGuestAuth, BasicUser},
    generated::{DatabaseErr, UIDNotFoundErr},
};

// To use this type of authentication, please specify a user_id resource on the request which
// refers to the Uuid of the user which is being accessed
pub struct UserViewAuth {}

impl BasicAuthTrait for UserViewAuth {
    async fn authenticate(
        user: BasicUser<Self>,
        req: actix_web::HttpRequest,
    ) -> crate::Result<BasicUser<Self>> {
        let user_id = req
            .match_info()
            .get("user_id")
            .expect("The route scope is missing a {user_id} path parameter");

        // TODO: This is currently just a bogus check to see if the authorization design works.

        if user.role == RoleType::Admin || user.uid.to_string() == user_id {
            Ok(user)
        } else {
            Err(Error::SessionUnauthorized)
        }
    }
}

// assuming we don't want to filter out specific users depending on the callers permissions

#[api_operation(
    tag = "user",
    summary = "Get a view of all users",
    security_scope(name = "session-id", scope = "admin",)
)]
pub async fn user_list(
    db: Data<Database>,
    _user: BasicUser<BasicAdminAuth>,
) -> crate::Result<Json<Vec<UserView>>> {
    let users = get_all_user_views(&db).await.context(DatabaseErr)?;

    Ok(Json(users))
}

#[api_operation(
    tag = "user",
    summary = "View a user by their user id",
    security_scope(name = "session-id", scope = "user")
)]
pub async fn view_user(
    db: Data<Database>,
    _user: BasicUser<UserViewAuth>,
    user_id: web::Path<String>,
) -> crate::Result<Json<UserView>> {
    let uid = user_id.into_inner();
    let uuid = Uuid::from_str(&uid).map_err(|_| Error::InvalidUid { uid: uid.clone() })?;
    let user = get_user_view_by_id(uuid, &db)
        .await
        .context(DatabaseErr)?
        .context(UIDNotFoundErr { uid })?;

    Ok(Json(user))
}

#[api_operation(
    tag = "user",
    summary = "View self as logged in user",
    security_scope(name = "session-id",)
)]
pub async fn view_me(
    db: Data<Database>,
    user: BasicUser<BasicGuestAuth>,
) -> crate::Result<Json<UserView>> {
    let user = get_user_view_by_id(user.uid, &db)
        .await
        .context(DatabaseErr)?;

    match user {
        Some(user) => Ok(Json(user)),
        None => {
            tracing::error!("User session passed but user object not found");
            Err(Error::GenericInternalError)
        }
    }
}
