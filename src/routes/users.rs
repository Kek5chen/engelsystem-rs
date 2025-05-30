use std::str::FromStr;

use actix_web::{
    get,
    web::{self, Data, Html}, Responder,
};
use engelsystem_rs_db::{
    role::RoleType,
    user::{get_all_user_views, get_user_view_by_id},
    DatabaseConnection,
};
use snafu::ResultExt;
use tera::{Context, Tera};
use uuid::Uuid;

use crate::{
    authorize_middleware::{BasicAdminAuth, BasicAuthTrait, BasicUser},
    generated::{DatabaseErr, TemplateErr},
    Error,
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
    templates: Data<Tera>,
    db: Data<DatabaseConnection>,
    _user: BasicUser<BasicAdminAuth>,
) -> crate::Result<impl Responder> {
    let users = get_all_user_views(&db).await.context(DatabaseErr)?;
    let mut context = Context::new();

    context.insert("org", "Real Org");
    context.insert("users", &users);

    Ok(Html::new(
        templates
            .render("user_list.html", &context)
            .context(TemplateErr)?,
    ))
}

#[get("/users/{user_id}")]
pub async fn view_user(
    templates: Data<Tera>,
    db: Data<DatabaseConnection>,
    _user: BasicUser<UserViewAuth>,
    user_id: web::Path<String>,
) -> crate::Result<impl Responder> {
    let uid = user_id.into_inner();
    let uid = Uuid::from_str(&uid).map_err(|_| Error::InvalidUid { uid })?;
    let user = get_user_view_by_id(uid, &db).await.context(DatabaseErr)?;
    let mut context = Context::new();

    context.insert("org", "Real Org");
    context.insert("user", &user);

    Ok(Html::new(
        templates
            .render("user_view.html", &context)
            .context(TemplateErr)?,
    ))
}
