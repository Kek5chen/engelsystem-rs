use apistos::ApiComponent;
use schemars::JsonSchema;
use sea_orm::prelude::*;
use sea_orm::{DeriveEntityModel, prelude::async_trait::async_trait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[sea_orm(table_name = "shift")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeUtc,
    pub created_by: Uuid,
    pub managed_by: Option<Uuid>,
    pub starts_at: DateTimeUtc,
    pub ends_at: DateTimeUtc,
    pub name: String,
    pub description: Option<String>,
    pub angels_needed: u32,
    pub angel_type_id: Option<u32>,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::angel_type::Entity",
        from = "Column::AngelTypeId",
        to = "super::angel_type::Column::Id"
    )]
    AngelTypeId,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::ManagedBy",
        to = "super::user::Column::Id"
    )]
    ManagedBy,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::CreatedBy",
        to = "super::user::Column::Id"
    )]
    CreatedBy,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}
