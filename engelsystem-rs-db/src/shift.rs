use entity::intern::*;
use sea_orm::{prelude::*, JoinType, QueryOrder, QuerySelect};

pub async fn add_shift(
    shift: shift::ActiveModel,
    db: &DatabaseConnection,
) -> crate::Result<shift::Model> {
    Ok(shift.insert(db).await?)
}

pub async fn get_shifts_by_user(
    user_id: Uuid,
    limit: Option<u32>,
    include_expired: bool,
    include_started: bool,
    db: &DatabaseConnection,
) -> crate::Result<Vec<shift::Model>> {
    let mut select = Shift::find()
        .join_rev(JoinType::InnerJoin, user_shift::Relation::Shift.def())
        .filter(user_shift::Column::UserId.eq(user_id))
        .order_by_asc(shift::Column::StartsAt);

    if let Some(limit) = limit {
        select = select.limit(limit as u64);
    }

    if !include_expired {
        select = select.filter(Expr::col(shift::Column::EndsAt).gt(Expr::current_timestamp()))
    }

    if !include_started {
        select = select.filter(Expr::col(shift::Column::StartsAt).gt(Expr::current_timestamp()))
    }

    Ok(select.all(db).await?)
}
