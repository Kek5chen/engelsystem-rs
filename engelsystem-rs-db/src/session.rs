use std::collections::HashMap;

use entity::{ActiveSession, Session};
use rand::distr::Alphanumeric;
use rand::Rng;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};
use sea_orm::entity::prelude::*;
use snafu::ResultExt;

use snafu::Snafu;
use time::{Duration, OffsetDateTime};
use tracing::debug;

type SessionResult<T, E = SessionError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(Err)),visibility(pub(crate)))]
pub enum SessionError {
    #[snafu(transparent)]
    Database {
        source: sea_orm::DbErr,
    },

    #[snafu(display("Session Data couldn't be deserialized: {source}"))]
    SessionDeserialize {
        source: serde_json::Error,
    },

    #[snafu(display("Session Data couldn't be serialized: {source}"))]
    SessionSerialize {
        source: serde_json::Error,
    },

    #[snafu(display("Session was not found"))]
    SessionNotFound,
}

pub async fn load_session(
    db: &DatabaseConnection,
    session_key: &str
) -> SessionResult<Option<HashMap<String, String>>>
{
    debug!("Loading session...");

    let session = match Session::find_by_id(session_key)
        .one(db)
        .await? 
    {
        Some(session) => session,
        None => return Ok(None),
    };

    if session.expires_at < OffsetDateTime::now_utc() {
        session.delete(db).await?;
        return Ok(None);
    }

    Ok(Some(serde_json::from_str(&session.data).context(SessionDeserializeErr)?))
}

pub async fn save_session(
    db: &DatabaseConnection,
    session_state: HashMap<String, String>,
    ttl: &Duration,
) -> SessionResult<String>
{
    debug!("Saving session...");

    let data = serde_json::to_string(&session_state).context(SessionSerializeErr)?;
    let expires_at = time::OffsetDateTime::now_utc() + *ttl;
    let session_key: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(256)
        .map(char::from)
        .collect();

    ActiveSession {
        id: Set(session_key.clone()),
        created_at: NotSet,
        data: Set(data),
        expires_at: Set(expires_at),
    }.insert(db)
    .await?;

    Ok(session_key)
}

pub async fn update_session(
    db: &DatabaseConnection,
    session_key: &str,
    session_state: HashMap<String, String>,
    ttl: &Duration,
) -> SessionResult<()>
{
    debug!("Updating session...");

    let data = serde_json::to_string(&session_state).context(SessionSerializeErr)?;
    let expires_at = OffsetDateTime::now_utc() + *ttl;

    let mut session = Session::find_by_id(session_key)
        .one(db)
        .await?
        .ok_or(SessionError::SessionNotFound)?
        .into_active_model();

    session.data = Set(data);
    session.expires_at = Set(expires_at);

    session.save(db).await?;

    Ok(())
}

pub async fn update_session_ttl(
    db: &DatabaseConnection,
    session_key: &str,
    ttl: &Duration,
) -> SessionResult<()>
{
    debug!("Updating session TTL...");

    let mut session = Session::find_by_id(session_key)
        .one(db)
        .await?
        .ok_or(SessionError::SessionNotFound)?
        .into_active_model();

    session.expires_at = Set(OffsetDateTime::now_utc() + *ttl);
    session.save(db).await?;

    Ok(())
}

pub async fn delete_session(
    db: &DatabaseConnection,
    session_key: &str,
) -> SessionResult<()> {
    debug!("Deleting session...");

    if Session::delete_by_id(session_key)
        .exec(db)
        .await?
        .rows_affected == 0 {
        return Err(SessionError::SessionNotFound);
    }

    Ok(())
}
