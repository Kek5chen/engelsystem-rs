pub type Result<T, E = Error> = std::result::Result<T, E>;

pub type Error = anyhow::Error;
