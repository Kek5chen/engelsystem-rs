use entity::intern::*;
use sea_orm::entity::*;
use sea_orm_migration::{prelude::*, schema::*};

macro_rules! drop_table {
    ($manager:ident, $( $entity:ty ),*) => {
        $(
            $manager
                .drop_table(Table::drop().table(<$entity>::Table).to_owned())
                .await?;
        )*
    };
}

const ROLE_NAMES: [&str; 3] = ["Guest", "User", "Administrator"];

const PERMISSION_NAMES: [&str; 2] = ["AddUser", "DeleteUser"];

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // =============================
        // Permission
        // =============================

        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(pk_auto(Permission::Id))
                    .col(string_uniq(Permission::Name))
                    .to_owned(),
            )
            .await?;

        // =============================
        // Role
        // =============================

        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(pk_auto(Role::Id))
                    .col(string_uniq(Role::Name))
                    .to_owned(),
            )
            .await?;

        seed_permissions(manager.get_connection()).await?;
        seed_roles(manager.get_connection()).await?;

        // =============================
        // User
        // =============================

        let mut user_role_foreign_key = ForeignKey::create()
            .name("FK-user-role_id")
            .from(User::Table, User::RoleId)
            .to(Role::Table, Role::Id)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id))
                    .col(integer_uniq(User::MemberId))
                    .col(timestamp(User::CreatedAt).default(Expr::current_timestamp()))
                    .col(string_uniq(User::Username))
                    .col(string_uniq(User::Email))
                    .col(string_null(User::PasswordHash))
                    .col(integer(User::RoleId))
                    .col(integer(User::ShiftTime).default(Expr::val(0)))
                    .col(integer(User::Points).default(Expr::val(0)))
                    .foreign_key(&mut user_role_foreign_key)
                    .to_owned(),
            )
            .await?;

        // ============================
        // Session
        // ============================

        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(string_len(Session::Id, 4064).primary_key())
                    .col(timestamp(Session::CreatedAt).default(Expr::current_timestamp()))
                    .col(string_null(Session::Data))
                    .col(date_time_null(Session::ExpiresAt))
                    .to_owned(),
            )
            .await?;

        // ============================
        // Angel Type
        // ============================

        manager
            .create_table(
                Table::create()
                    .table(AngelType::Table)
                    .if_not_exists()
                    .col(pk_auto(AngelType::Id))
                    .col(timestamp(AngelType::CreatedAt).default(Expr::current_timestamp()))
                    .col(string_uniq(AngelType::Name))
                    .col(boolean(AngelType::NeedsIntroduction))
                    .to_owned(),
            )
            .await?;

        // ============================
        // Shift
        // ============================

        let mut shift_angel_type = ForeignKey::create()
            .name("FK-shift-angel_type")
            .from(Shift::Table, Shift::AngelTypeId)
            .to(AngelType::Table, AngelType::Id)
            .to_owned();

        let mut shift_managed_by = ForeignKey::create()
            .name("FK-shift-managed_by")
            .from(Shift::Table, Shift::ManagedBy)
            .to(User::Table, User::Id)
            .to_owned();

        let mut shift_created_by = ForeignKey::create()
            .name("FK-shift-created_by")
            .from(Shift::Table, Shift::CreatedBy)
            .to(User::Table, User::Id)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Shift::Table)
                    .if_not_exists()
                    .col(uuid(Shift::Id).primary_key())
                    .col(timestamp(Shift::CreatedAt).default(Expr::current_timestamp()))
                    .col(uuid(Shift::CreatedBy))
                    .col(uuid_null(Shift::ManagedBy))
                    .col(timestamp(Shift::StartsAt))
                    .col(timestamp(Shift::EndsAt))
                    .col(string(Shift::Name))
                    .col(string_null(Shift::Description))
                    .col(integer(Shift::AngelsNeeded))
                    .col(integer(Shift::AngelTypeId))
                    .foreign_key(&mut shift_angel_type)
                    .foreign_key(&mut shift_managed_by)
                    .foreign_key(&mut shift_created_by)
                    .to_owned(),
            )
            .await?;

        // ============================
        // User -> Shift
        // ============================

        let mut user_shift_user = ForeignKey::create()
            .name("FK-user_shift-user")
            .from(UserShift::Table, UserShift::UserId)
            .to(User::Table, User::Id)
            .to_owned();

        let mut user_shift_shift = ForeignKey::create()
            .name("FK-user_shift-shift")
            .from(UserShift::Table, UserShift::ShiftId)
            .to(Shift::Table, Shift::Id)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(UserShift::Table)
                    .if_not_exists()
                    .col(uuid(UserShift::UserId))
                    .col(integer(UserShift::ShiftId))
                    .primary_key(
                        Index::create()
                            .col(UserShift::UserId)
                            .col(UserShift::ShiftId),
                    )
                    .foreign_key(&mut user_shift_user)
                    .foreign_key(&mut user_shift_shift)
                    .to_owned(),
            )
            .await?;

        // ============================
        // Role -> Permission
        // ============================

        let mut role_permission_user = ForeignKey::create()
            .name("FK-role_permission-user")
            .from(RolePermission::Table, RolePermission::RoleId)
            .to(Role::Table, Role::Id)
            .to_owned();

        let mut role_permission_permission = ForeignKey::create()
            .name("FK-role_permission-permission")
            .from(RolePermission::Table, RolePermission::PermissionId)
            .to(Permission::Table, Permission::Id)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(RolePermission::Table)
                    .if_not_exists()
                    .col(integer(RolePermission::RoleId))
                    .col(integer(RolePermission::PermissionId))
                    .col(boolean(RolePermission::Enabled))
                    .primary_key(
                        Index::create()
                            .col(RolePermission::RoleId)
                            .col(RolePermission::PermissionId),
                    )
                    .foreign_key(&mut role_permission_user)
                    .foreign_key(&mut role_permission_permission)
                    .to_owned(),
            )
            .await?;

        // ============================
        // User -> Angel Type
        // ============================

        let mut user_angel_type_user = ForeignKey::create()
            .name("FK-user_angel_type-user")
            .from(UserAngelType::Table, UserAngelType::UserId)
            .to(User::Table, User::Id)
            .to_owned();

        let mut user_angel_type_angel_type = ForeignKey::create()
            .name("FK-user_angel_type-angel_type")
            .from(UserAngelType::Table, UserAngelType::AngelTypeId)
            .to(AngelType::Table, AngelType::Id)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(UserAngelType::Table)
                    .if_not_exists()
                    .col(uuid(UserAngelType::UserId))
                    .col(integer(UserAngelType::AngelTypeId))
                    .primary_key(
                        Index::create()
                            .col(UserAngelType::UserId)
                            .col(UserAngelType::AngelTypeId),
                    )
                    .foreign_key(&mut user_angel_type_user)
                    .foreign_key(&mut user_angel_type_angel_type)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_table!(
            manager,
            User,
            Permission,
            Role,
            Session,
            AngelType,
            Shift,
            UserShift,
            RolePermission,
            UserAngelType
        );

        Ok(())
    }
}

async fn seed_permissions(conn: &SchemaManagerConnection<'_>) -> Result<(), DbErr> {
    for permission in PERMISSION_NAMES {
        permission::ActiveModel {
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
        role::ActiveModel {
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
    MemberId,
    CreatedAt,
    Username,
    Email,
    PasswordHash,
    RoleId,

    ShiftTime,
    Points,
}

#[derive(DeriveIden)]
pub enum Session {
    Table,
    Id,
    CreatedAt,
    Data,
    ExpiresAt,
}

#[derive(DeriveIden)]
pub enum Shift {
    Table,
    Id,
    CreatedAt,
    CreatedBy,
    ManagedBy,
    StartsAt,
    EndsAt,
    Name,
    Description,
    AngelsNeeded,
    AngelTypeId,
}

#[derive(DeriveIden)]
pub enum UserShift {
    Table,
    UserId,
    ShiftId,
}

#[derive(DeriveIden)]
pub enum AngelType {
    Table,
    Id,
    CreatedAt,
    Name,
    NeedsIntroduction,
}

#[derive(DeriveIden)]
pub enum RolePermission {
    Table,
    RoleId,
    PermissionId,
    Enabled,
}

#[derive(DeriveIden)]
pub enum UserAngelType {
    Table,
    UserId,
    AngelTypeId,
}
