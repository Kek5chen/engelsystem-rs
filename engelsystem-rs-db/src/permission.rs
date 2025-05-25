use entity::*;
use sea_orm::prelude::*;

pub async fn get_perm_count(db: &DatabaseConnection) -> Result<u64, DbErr> {
    Permission::find().count(db).await
}

pub async fn get_permission_by_name(name: &str, db: &DatabaseConnection) -> Result<Option<permission::Model>, DbErr> {
    Permission::find()
        .filter(permission::Column::Name.eq(name))
        .one(db)
        .await
}

#[cfg(test)]
mod tests {
    use crate::tests::connect_and_migrate_dummy;
    use super::*;
    use test_log::test;

    #[test(tokio::test)]
    async fn permission_seeding() {
        let db = connect_and_migrate_dummy().await.unwrap();
        let perm_count = get_perm_count(&db).await.unwrap();
        assert!(perm_count > 0);
    }
}
