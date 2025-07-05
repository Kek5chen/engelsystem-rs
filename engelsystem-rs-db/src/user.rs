use std::time::Duration;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::PasswordHasher;
use argon2::{password_hash::SaltString, Argon2};
use argon2::{PasswordHash, PasswordVerifier};
use entity::public::{self};
use sea_orm::{prelude::*, ActiveValue::*, IntoActiveModel, Iterable, QuerySelect, SelectColumns};
use snafu::ResultExt;
use tracing::error;

use crate::role::RoleType;
use crate::Error;
use entity::intern::*;

pub async fn get_all_guests(db: &DatabaseConnection) -> crate::Result<Vec<user::Model>> {
    Ok(User::find()
        .filter(user::Column::RoleId.eq(RoleType::Guest as u32))
        .all(db)
        .await?)
}

pub async fn get_all_users(db: &DatabaseConnection) -> crate::Result<Vec<user::Model>> {
    Ok(User::find().all(db).await?)
}

pub async fn get_all_admins(db: &DatabaseConnection) -> crate::Result<Vec<user::Model>> {
    Ok(User::find()
        .filter(user::Column::RoleId.eq(RoleType::Admin as u32))
        .all(db)
        .await?)
}

pub async fn get_all_user_views(db: &DatabaseConnection) -> crate::Result<Vec<user::View>> {
    Ok(User::find()
        .inner_join(Role)
        .column_as(role::Column::Name, "role")
        .into_model::<user::View>()
        .all(db)
        .await?)
}

pub async fn get_user_count(db: &DatabaseConnection) -> crate::Result<u64> {
    Ok(User::find().count(db).await?)
}

pub async fn get_guest_count(db: &DatabaseConnection) -> crate::Result<u64> {
    Ok(User::find()
        .filter(user::Column::RoleId.eq(RoleType::Guest as u32))
        .count(db)
        .await?)
}

pub async fn get_admin_count(db: &DatabaseConnection) -> crate::Result<u64> {
    Ok(User::find()
        .filter(user::Column::RoleId.eq(RoleType::Admin as u32))
        .count(db)
        .await?)
}

pub async fn get_user_by_id(
    uid: Uuid,
    db: &DatabaseConnection,
) -> crate::Result<Option<user::Model>> {
    Ok(User::find_by_id(uid).one(db).await?)
}

pub async fn get_user_by_name(
    name: &str,
    db: &DatabaseConnection,
) -> crate::Result<Option<user::Model>> {
    Ok(User::find()
        .filter(user::Column::Username.eq(name))
        .one(db)
        .await?)
}

pub async fn get_user_id_by_name(
    name: &str,
    db: &DatabaseConnection,
) -> crate::Result<Option<Uuid>> {
    Ok(User::find()
        .filter(user::Column::Username.eq(name))
        .select_only()
        .column(user::Column::Id)
        .into_tuple()
        .one(db)
        .await?)
}

pub async fn get_angel_type_id_by_name(
    name: &str,
    db: &DatabaseConnection,
) -> crate::Result<Option<u32>> {
    Ok(AngelType::find()
        .filter(angel_type::Column::Name.eq(name))
        .select_only()
        .column(angel_type::Column::Id)
        .into_tuple()
        .one(db)
        .await?)
}

pub async fn update_user(
    uid: Uuid,
    changes: public::ActiveUser,
    db: &DatabaseConnection,
) -> crate::Result<Option<user::Model>> {
    let Some(user) = get_user_by_id(uid, db).await? else {
        return Err(Error::UserNotFound);
    };

    let mut user = user.into_active_model();

    for col in user::Column::iter() {
        if let Set(new) = changes.get(col) {
            if user.get(col).into_value().as_ref() != Some(&new) {
                user.set(col, new);
            }
        }
    }

    if user.is_changed() {
        Ok(Some(user.update(db).await?))
    } else {
        Ok(None)
    }
}

pub async fn get_user_view_by_id(
    uid: Uuid,
    db: &DatabaseConnection,
) -> crate::Result<Option<public::UserView>> {
    Ok(User::find_by_id(uid)
        .inner_join(Role)
        .column_as(role::Column::Name, "role")
        .into_model::<public::UserView>()
        .one(db)
        .await?)
}

pub fn hash_password(plain_password: &str) -> crate::Result<String> {
    let hasher = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = hasher
        .hash_password(plain_password.as_bytes(), &salt)
        .map_err(|_| Error::Hashing)?
        .to_string();
    Ok(password_hash)
}

fn verify_password(plain_password: &str, hashed_password: &str) -> bool {
    let hasher = Argon2::default();
    let hashed = match PasswordHash::new(hashed_password) {
        Ok(hashed) => hashed,
        Err(e) => {
            error!("Failed to parse hashed password from database: {e}");
            return false;
        }
    };

    hasher
        .verify_password(plain_password.as_bytes(), &hashed)
        .is_ok()
}

pub async fn verify_user(
    username: &str,
    plain_password: &str,
    db: &DatabaseConnection,
) -> Option<user::Model> {
    tokio::time::sleep(Duration::from_millis(rand::random_range(0..2000))).await;

    let user = match User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return None;
        }
        Err(e) => {
            error!("Error when retrieving user from database: {e}");
            return None;
        }
    };

    if verify_password(plain_password, &user.password_hash) {
        return Some(user);
    }

    None
}

pub async fn add_generic_user(
    username: impl Into<String>,
    email: impl Into<String>,
    plain_password: &str,
    role: RoleType,
    db: &DatabaseConnection,
) -> crate::Result<user::Model> {
    let password_hash = hash_password(plain_password)?;

    let member_id: Option<u32> = User::find()
        .select_only()
        .expr(Expr::col(user::Column::MemberId).max())
        .into_tuple()
        .one(db)
        .await?
        .unwrap_or(None);

    let member_id = member_id.unwrap_or(0) + 1;

    let result = user::ActiveModel {
        member_id: Set(member_id),
        username: Set(username.into()),
        role_id: Set(role as u32),
        email: Set(email.into()),
        password_hash: Set(password_hash),
        points: NotSet,
        ..Default::default()
    }
    .insert(db)
    .await;

    match result {
        Ok(model) => Ok(model),
        Err(DbErr::Exec(RuntimeErr::SqlxError(sea_orm::sqlx::error::Error::Database(e))))
            if e.is_unique_violation() =>
        {
            Err(Error::UserExists)
        }
        Err(e) => Err(e.into()),
    }
}

#[inline]
pub async fn add_guest(
    username: impl Into<String>,
    email: impl Into<String>,
    plain_password: &str,
    db: &DatabaseConnection,
) -> crate::Result<user::Model> {
    add_generic_user(username, email, plain_password, RoleType::Guest, db).await
}

#[inline]
pub async fn add_user(
    username: impl Into<String>,
    email: impl Into<String>,
    plain_password: &str,
    db: &DatabaseConnection,
) -> crate::Result<user::Model> {
    add_generic_user(username, email, plain_password, RoleType::User, db).await
}

#[inline]
pub async fn add_admin(
    username: impl Into<String>,
    email: impl Into<String>,
    plain_password: &str,
    db: &DatabaseConnection,
) -> crate::Result<user::Model> {
    add_generic_user(username, email, plain_password, RoleType::Admin, db).await
}

pub async fn get_role_by_username(
    username: &str,
    db: &DatabaseConnection,
) -> crate::Result<RoleType> {
    let Some(user) = User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await?
    else {
        return Err(Error::UsernameNotFound {
            username: username.to_owned(),
        });
    };

    Ok(RoleType::from_repr(user.role_id)
        .expect("FIXME: Expand the logic to be able to handle custom roles"))
}

pub async fn set_role_by_username(
    username: &str,
    role: RoleType,
    db: &DatabaseConnection,
) -> crate::Result<user::Model> {
    let Some(user) = User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await?
    else {
        return Err(Error::UsernameNotFound {
            username: username.to_owned(),
        });
    };

    let mut user = user.into_active_model();
    user.role_id = Set(role as u32);

    Ok(user.update(db).await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::connect_and_migrate_dummy;
    use test_log::test;

    #[test(tokio::test)]
    async fn single_user() {
        let db = connect_and_migrate_dummy().await.unwrap();

        let user = add_guest("Meow", "meow@meow.de", "awawa", &db)
            .await
            .unwrap();

        let mut all_users = get_all_users(&db).await.unwrap();

        let db_user = all_users.pop().unwrap();
        assert_eq!(all_users.pop(), None);
        assert_eq!(db_user, user);
        assert_ne!(db_user.password_hash, "awawa");
    }

    #[test(tokio::test)]
    async fn multi_user() {
        let db = connect_and_migrate_dummy().await.unwrap();

        let user = add_guest("Meow", "meow@meow.de", "awawa", &db)
            .await
            .unwrap();
        let user2 = add_guest("Meow2", "meow@meow.de", "awawa", &db)
            .await
            .unwrap();

        let all_users = get_all_users(&db).await.unwrap();

        assert!(all_users.contains(&user));
        assert!(all_users.contains(&user2));
        assert_eq!(all_users.len(), 2);
    }
}
