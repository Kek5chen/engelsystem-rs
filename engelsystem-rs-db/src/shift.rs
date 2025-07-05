use entity::intern::*;
use sea_orm::{JoinType, QuerySelect, prelude::*};

pub async fn add_shift(
    shift: shift::ActiveModel,
    db: &DatabaseConnection,
) -> crate::Result<shift::Model> {
    Ok(shift.insert(db).await?)
}

pub async fn get_shifts_by_user(
    user_id: Uuid,
    db: &DatabaseConnection,
) -> crate::Result<Vec<shift::Model>> {
    Ok(Shift::find()
        .join_rev(JoinType::InnerJoin, user_shift::Relation::Shift.def())
        .filter(user_shift::Column::UserId.eq(user_id))
        .all(db)
        .await?)
}
