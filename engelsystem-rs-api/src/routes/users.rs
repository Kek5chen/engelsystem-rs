use std::str::FromStr;

use actix_web::{
    HttpResponse, Responder, get,
    web::{self, Data},
};
use engelsystem_rs_db::{
    Database,
    role::RoleType,
    user::{get_all_user_views, get_user_view_by_id},
};
use snafu::ResultExt;
use uuid::Uuid;

use crate::{
    Error,
    authorize_middleware::{BasicAdminAuth, BasicAuthTrait, BasicGuestAuth, BasicUser},
    generated::DatabaseErr,
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

#[get("/users")]
pub async fn user_list(
    db: Data<Database>,
    _user: BasicUser<BasicAdminAuth>,
) -> crate::Result<impl Responder> {
    let users = get_all_user_views(&db).await.context(DatabaseErr)?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{user_id}")]
pub async fn view_user(
    db: Data<Database>,
    _user: BasicUser<UserViewAuth>,
    user_id: web::Path<String>,
) -> crate::Result<impl Responder> {
    let uid = user_id.into_inner();
    let uid = Uuid::from_str(&uid).map_err(|_| Error::InvalidUid { uid })?;
    let user = get_user_view_by_id(uid, &db).await.context(DatabaseErr)?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/me")]
pub async fn view_me(
    db: Data<Database>,
    user: BasicUser<BasicGuestAuth>,
) -> crate::Result<impl Responder> {
    let user = get_user_view_by_id(user.uid, &db)
        .await
        .context(DatabaseErr)?;

    Ok(match user {
        Some(user) => HttpResponse::Ok().json(&user),
        None => {
            tracing::error!("User session passed but user object not found");
            HttpResponse::InternalServerError().finish()
        }
    })
}
