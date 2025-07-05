use entity::intern::*;
use sea_orm::prelude::*;

pub async fn get_perm_count(db: &DatabaseConnection) -> crate::Result<u64> {
    Ok(Permission::find().count(db).await?)
}

pub async fn get_permission_by_name(
    name: &str,
    db: &DatabaseConnection,
) -> crate::Result<Option<permission::Model>> {
    let result = Permission::find()
        .filter(permission::Column::Name.eq(name))
        .one(db)
        .await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::connect_and_migrate_dummy;
    use test_log::test;

    #[test(tokio::test)]
    async fn permission_seeding() {
        let db = connect_and_migrate_dummy().await.unwrap();
        let perm_count = get_perm_count(&db).await.unwrap();
        assert!(perm_count > 0);
    }
}
