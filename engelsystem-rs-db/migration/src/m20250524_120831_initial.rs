use entity::*;
use sea_orm::entity::*;
use sea_orm_migration::{prelude::*, schema::*};

const ROLE_NAMES: [&str; 3] = [
    "Guest",
    "User",
    "Administrator",
];

const PERMISSION_NAMES: [&str; 2] = [
    "AddUser",
    "DeleteUser",
];

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // =============================
        // Permissions and Roles
        // =============================

        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(pk_auto(Permission::Id))
                    .col(string(Permission::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(pk_auto(Role::Id))
                    .col(string(Role::Name))
                    .to_owned(),
            )
            .await?;

        seed_permissions(manager.get_connection()).await?;
        seed_roles(manager.get_connection()).await?;

        // =============================
        // Users
        // =============================

        let mut user_role_foreign_key = ForeignKey::create()
            .name("FK_user_role_id")
            .from_tbl(User::Table)
            .from_col(User::RoleId)
            .to_tbl(Role::Table)
            .to_col(Role::Id)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id))
                    .col(timestamp(User::Created).default(Expr::current_timestamp()))
                    .col(string(User::Username))
                    .col(integer(User::RoleId))
                    .foreign_key(&mut user_role_foreign_key)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}

async fn seed_permissions(conn: &SchemaManagerConnection<'_>) -> Result<(), DbErr> {
    for permission in PERMISSION_NAMES {
        ActivePermission {
            id: NotSet,
            name: Set(permission.to_string()),
        }
        .insert(conn)
        .await?;
    }

    Ok(())
}

async fn seed_roles(conn: &SchemaManagerConnection<'_>) -> Result<(), DbErr> {
    for role in ROLE_NAMES {
        ActiveRole {
            id: NotSet,
            name: Set(role.to_string()),
        }
        .insert(conn)
        .await?;
    }

    Ok(())
}

#[derive(DeriveIden)]
pub enum Permission {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
pub enum Role {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Created,
    Username,
    RoleId,
}
