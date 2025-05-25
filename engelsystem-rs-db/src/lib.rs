pub mod error;
pub mod permission;
pub mod role;
pub mod user;
pub mod session;

pub use error::*;

use migration::MigratorTrait;
pub use sea_orm::DatabaseConnection;

pub async fn connect(connection_string: &str) -> crate::Result<DatabaseConnection> {
    Ok(sea_orm::Database::connect(connection_string).await?)
}

pub async fn migrate(db: DatabaseConnection) -> crate::Result<DatabaseConnection> {
    migration::Migrator::up(&db, None).await?;
    Ok(db)
}

pub async fn connect_and_migrate(connection_string: &str) -> crate::Result<DatabaseConnection> {
    migrate(connect(connection_string).await?).await
}

#[cfg(test)]
mod tests {
    use crate::migrate;
    use sea_orm::DatabaseConnection;
    use test_log::test;

    pub(crate) async fn connect_dummy() -> crate::Result<DatabaseConnection> {
        Ok(sea_orm::Database::connect("sqlite::memory:").await?)
    }

    pub(crate) async fn connect_and_migrate_dummy() -> crate::Result<DatabaseConnection> {
        migrate(connect_dummy().await?).await
    }

    #[test(tokio::test)]
    async fn new_db_and_migrate() {
        connect_and_migrate_dummy().await.unwrap();
    }
}
