use actix_web::{
    get, http::header, web::{self, Data, Html}, HttpRequest, Responder
};
use snafu::ResultExt;
use tera::{Context, Tera};

use crate::{
    generated::{BackendErr, TemplateErr},
    Error,
};

#[get("/users")]
pub async fn user_list(
    templates: Data<Tera>,
    client: Data<reqwest::Client>,
    req: HttpRequest,
) -> crate::Result<impl Responder> {
    let Some(auth_header) = req.headers().get(header::AUTHORIZATION) else {
        return Err(Error::Unauthorized);
    };

    const USERS_URL: &str = "http://127.0.0.1:8081/users";
    let users: serde_json::Value = client.get(USERS_URL)
        .header(reqwest::header::AUTHORIZATION, auth_header.to_str().map_err(|_| Error::Unauthorized)?)
        .send()
        .await
        .context(BackendErr)?
        .json()
        .await
        .context(BackendErr)?;

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
    user_id: web::Path<String>,
    client: Data<reqwest::Client>,
    req: HttpRequest,
) -> crate::Result<impl Responder> {
    let Some(auth_header) = req.headers().get(header::AUTHORIZATION) else {
        return Err(Error::Unauthorized);
    };

    const USERS_URL: &str = "http://127.0.0.1:8081/users";
    let users: serde_json::Value = client.get(format!("{USERS_URL}/{user_id}"))
        .header(reqwest::header::AUTHORIZATION, auth_header.to_str().map_err(|_| Error::Unauthorized)?)
        .send()
        .await
        .context(BackendErr)?
        .json()
        .await
        .context(BackendErr)?;

    let mut context = Context::new();

    context.insert("org", "Real Org");
    context.insert("user", &users);

    Ok(Html::new(
        templates
            .render("user_view.html", &context)
            .context(TemplateErr)?,
    ))
}
