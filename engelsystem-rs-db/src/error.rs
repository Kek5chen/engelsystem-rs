use snafu::Snafu;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(Err)),module(generated),visibility(pub(crate)))]
pub enum Error {
    #[snafu(transparent)]
    Database {
        source: sea_orm::DbErr,
    },

    #[snafu(display("User with this mail already exists"))]
    UserExists,

    #[snafu(display("Hashing Error"))]
    Hashing,
}
