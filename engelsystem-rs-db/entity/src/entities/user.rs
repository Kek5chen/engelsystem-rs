use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub member_id: u32,
    pub created_at: DateTimeUtc,
    #[sea_orm(unique_key)]
    pub username: String,
    #[sea_orm(unique_key)]
    pub email: String,
    pub password_hash: String,
    #[sea_orm(default_value = 0)]
    pub role_id: u32,

    #[sea_orm(default_value = 0)]
    pub points: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::role::Entity",
        from = "Column::RoleId",
        to = "super::role::Column::Id",
    )]
    Role,
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl Related<super::shift::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_shift::Relation::Shift.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_shift::Relation::User.def())
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _: &C, _: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait
    {
        if self.id.is_not_set() {
            self.id = Set(Uuid::new_v4());
        }

        Ok(self) 
    }
}

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct View {
    pub id: Uuid,
    pub member_id: u32,
    pub created_at: DateTimeUtc,
    pub username: String,
    pub email: String,
    pub role: String,

    pub points: u32,
}

