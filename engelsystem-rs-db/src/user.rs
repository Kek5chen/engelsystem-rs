use entity::*;
use sea_orm::{prelude::*, ActiveValue::*};

use crate::role::{ADMIN_ROLE_ID, GUEST_ROLE_ID, USER_ROLE_ID};

pub async fn get_all_guests(db: &DatabaseConnection) -> Result<Vec<user::Model>, DbErr> {
    Ok(user::Entity::find().filter(user::Column::RoleId.eq(GUEST_ROLE_ID)).all(db).await?)
}

pub async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<user::Model>, DbErr> {
    Ok(user::Entity::find().all(db).await?)
}

pub async fn get_all_admins(db: &DatabaseConnection) -> Result<Vec<user::Model>, DbErr> {
    Ok(user::Entity::find().filter(user::Column::RoleId.eq(ADMIN_ROLE_ID)).all(db).await?)
}

pub async fn add_guest(username: impl Into<String>, db: &DatabaseConnection) -> Result<user::Model, DbErr> {
    ActiveUser {
        username: Set(username.into()),
        role_id: Set(GUEST_ROLE_ID),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn add_user(username: impl Into<String>, db: &DatabaseConnection) -> Result<user::Model, DbErr> {
    ActiveUser {
        username: Set(username.into()),
        role_id: Set(USER_ROLE_ID),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn add_admin(username: impl Into<String>, db: &DatabaseConnection) -> Result<user::Model, DbErr> {
    ActiveUser {
        username: Set(username.into()),
        role_id: Set(ADMIN_ROLE_ID),
        ..Default::default()
    }
    .insert(db)
    .await
}

#[cfg(test)]
mod tests {
    use crate::tests::connect_and_migrate_dummy;
    use super::*;
    use test_log::test;

    #[test(tokio::test)]
    async fn single_user() {
        let db = connect_and_migrate_dummy().await.unwrap();
        
        let user = add_guest("Meow", &db).await.unwrap();

        let mut all_users = get_all_users(&db).await.unwrap();

        assert_eq!(all_users.pop().unwrap(), user);
        assert_eq!(all_users.pop(), None);
    }

    #[test(tokio::test)]
    async fn multi_user() {
        let db = connect_and_migrate_dummy().await.unwrap();
        
        let user  = add_guest("Meow", &db).await.unwrap();
        let user2 = add_guest("Meow2", &db).await.unwrap();

        let all_users = get_all_users(&db).await.unwrap();

        assert!(all_users.contains(&user));
        assert!(all_users.contains(&user2));
        assert_eq!(all_users.len(), 2);
    }
}
