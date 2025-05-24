pub mod error;
pub mod user;
pub mod role;

pub use error::*;

use migration::MigratorTrait;
use sea_orm::DatabaseConnection;

pub async fn connect() -> Result<DatabaseConnection, sea_orm::DbErr> {
    todo!()
}

pub async fn migrate(db: DatabaseConnection) -> Result<DatabaseConnection, migration::DbErr> {
    migration::Migrator::up(&db, None).await?;
    Ok(db)
}

pub async fn connect_and_migrate() -> Result<DatabaseConnection, sea_orm::DbErr> {
    migrate(connect().await?).await
}



#[cfg(test)]
mod tests {
    use sea_orm::DatabaseConnection;
    use test_log::test;
    use crate::migrate;

    pub(crate) async fn connect_dummy() -> Result<DatabaseConnection, sea_orm::DbErr> {
        sea_orm::Database::connect("sqlite::memory:").await
    }

    pub(crate) async fn connect_and_migrate_dummy() -> Result<DatabaseConnection, sea_orm::DbErr> {
        migrate(connect_dummy().await?).await
    }

    #[test(tokio::test)]
    async fn new_db_and_migrate() {
        connect_and_migrate_dummy().await.unwrap();
    }
}
