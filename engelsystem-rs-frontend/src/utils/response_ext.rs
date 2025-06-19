use actix_web::{HttpResponse, HttpResponseBuilder, http::header::ContentType};

pub trait HtmlResponseExt {
    fn html(self, html: impl Into<String>) -> HttpResponse;
}

impl HtmlResponseExt for HttpResponseBuilder {
    fn html(mut self, html: impl Into<String>) -> HttpResponse {
        self.content_type(ContentType::html()).body(html.into())
    }
}
