use tera::Context;

pub struct BaseData<'a> {
    org: &'a str,
    logged_in: bool,
}

impl<'a> BaseData<'a> {
    pub fn new(org: &'a str, logged_in: bool) -> Self {
        Self { org, logged_in }
    }

    pub fn insert(self, ctx: &mut Context) {
        ctx.insert("org", &self.org);
        ctx.insert("logged_in", &self.logged_in);
    }
}
