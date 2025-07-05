use sea_orm::DeriveEntityModel;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "angel_type")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u32,
    pub created_at: DateTimeUtc,
    pub name: String,
    pub needs_introduction: bool,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}
