use sea_orm::{prelude::async_trait::async_trait, DeriveEntityModel};
use uuid::Uuid;
use sea_orm::prelude::*;


#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "shift")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeUtc,
    pub managed_by: Uuid,
    pub starts_at: DateTimeUtc,
    pub ends_at: DateTimeUtc,
    pub name: String,
    pub description: Option<String>,
    pub angels_needed: u32,
    pub angel_type_id: u32,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::angel_type::Entity",
        from = "Column::AngelTypeId",
        to = "super::angel_type::Column::Id",
    )]
    AngelTypeId,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::ManagedBy",
        to = "super::user::Column::Id",
    )]
    ManagedBy,
}

impl Related<super::angel_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AngelTypeId.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ManagedBy.def()
    }
}


#[async_trait]
impl ActiveModelBehavior for ActiveModel {}
