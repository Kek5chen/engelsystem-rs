use snafu::Snafu;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(Err)), module(generated), visibility(pub(crate)))]
pub enum Error {
    #[snafu(transparent)]
    Database { source: sea_orm::DbErr },

    #[snafu(display("The requested user was not found"))]
    UserNotFound,

    #[snafu(display("User with this mail already exists"))]
    UserExists,

    #[snafu(display("There's no user with the username {username:?}"))]
    UsernameNotFound { username: String },

    #[snafu(display("Hashing Error"))]
    Hashing,
}
