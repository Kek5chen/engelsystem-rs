use actix_web::{HttpResponse, HttpResponseBuilder, http::header::ContentType};

pub trait ActixResponseExt {
    fn html(self, html: impl Into<String>) -> HttpResponse;
    fn redirect_to(&mut self, url: &str) -> &mut Self;
    fn expire_session(&mut self) -> &mut Self;
    fn session_cookie(&mut self, session_id: impl Into<String>) -> &mut Self;
}

impl ActixResponseExt for HttpResponseBuilder {
    fn html(mut self, html: impl Into<String>) -> HttpResponse {
        self.content_type(ContentType::html()).body(html.into())
    }

    fn redirect_to(&mut self, url: &str) -> &mut Self {
        self.insert_header((actix_web::http::header::LOCATION, url))
    }

    fn expire_session(&mut self) -> &mut Self {
        let mut expire_cookie = actix_web::cookie::Cookie::new("session-id", "");
        expire_cookie.make_removal();

        self.cookie(expire_cookie);

        self
    }

    fn session_cookie(&mut self, session_id: impl Into<String>) -> &mut Self {
        let cookie = actix_web::cookie::Cookie::build("session-id", session_id.into())
            .secure(true)
            .http_only(true)
            .finish();

        self.cookie(cookie)
    }
}
