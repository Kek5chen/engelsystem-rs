use askama::Template;

#[derive(Template)]
#[template(path = "landing.html")]
pub struct Index<'a> {
    pub org: &'a str,
    pub rows: Vec<(&'a str, u64)>,
}
