use entity::*;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, FromRepr, IntoStaticStr};

#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    FromRepr,
    EnumString,
    IntoStaticStr,
)]
#[repr(u32)]
#[strum(ascii_case_insensitive)]
pub enum RoleType {
    Guest = 1,
    User = 2,
    Admin = 3,
}

impl RoleType {
    pub fn from_or_default(value: u32) -> RoleType {
        Self::from_repr(value).unwrap_or(RoleType::Guest)
    }
}

pub async fn get_role_count(db: &DatabaseConnection) -> crate::Result<u64> {
    Ok(Role::find().count(db).await?)
}

pub async fn get_role_by_name(
    name: &str,
    db: &DatabaseConnection,
) -> crate::Result<Option<role::Model>> {
    Ok(Role::find()
        .filter(role::Column::Name.eq(name))
        .one(db)
        .await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::connect_and_migrate_dummy;
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

        let guest = Role::find_by_id(RoleType::Guest as u32)
            .one(&db)
            .await
            .unwrap()
            .unwrap();
        let user = Role::find_by_id(RoleType::User as u32)
            .one(&db)
            .await
            .unwrap()
            .unwrap();
        let admin = Role::find_by_id(RoleType::Admin as u32)
            .one(&db)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(guest.name, "Guest");
        assert_eq!(user.name, "User");
        assert_eq!(admin.name, "Administrator");
    }
}
