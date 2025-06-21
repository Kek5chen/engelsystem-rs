use sea_orm::prelude::async_trait::async_trait;
use sea_orm::DeriveEntityModel;
use sea_orm::prelude::*;


#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "user_shift")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    pub shift_id: Uuid,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}
