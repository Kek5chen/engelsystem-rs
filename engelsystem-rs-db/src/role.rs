use entity::*;
use sea_orm::prelude::*;

pub const GUEST_ROLE_ID: u32 = 1;
pub const USER_ROLE_ID: u32 = 2;
pub const ADMIN_ROLE_ID: u32 = 3;

pub async fn get_role_count(db: &DatabaseConnection) -> Result<u64, DbErr> {
    Role::find().count(db).await
}

pub async fn get_role_by_name(name: &str, db: &DatabaseConnection) -> Result<Option<role::Model>, DbErr> {
    Role::find()
        .filter(role::Column::Name.eq(name))
        .one(db)
        .await
}

#[cfg(test)]
mod tests {
    use crate::tests::connect_and_migrate_dummy;
    use super::*;
    use test_log::test;

    #[test(tokio::test)]
    async fn role_seeding() {
        let db = connect_and_migrate_dummy().await.unwrap();
        let role_count = get_role_count(&db).await.unwrap();
        assert!(role_count > 0);
    }

    #[test(tokio::test)]
    async fn ensure_base_role_indices() {
        let db = connect_and_migrate_dummy().await.unwrap();

        let guest = Role::find_by_id(GUEST_ROLE_ID).one(&db).await.unwrap().unwrap();
        let user  = Role::find_by_id(USER_ROLE_ID).one(&db).await.unwrap().unwrap();
        let admin = Role::find_by_id(ADMIN_ROLE_ID).one(&db).await.unwrap().unwrap();

        assert_eq!(guest.name, "Guest");
        assert_eq!(user.name,  "User");
        assert_eq!(admin.name, "Administrator");
    }
}
