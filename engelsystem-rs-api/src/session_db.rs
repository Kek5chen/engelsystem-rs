use std::collections::HashMap;

use actix_session::storage::{LoadError, SaveError, SessionKey, SessionStore, UpdateError};
use actix_web::{cookie::time::Duration, web::Data};
use engelsystem_rs_db::{
    DatabaseConnection,
    session::{delete_session, load_session, save_session, update_session, update_session_ttl},
};
use tracing::error;

pub(crate) type SessionState = HashMap<String, String>;

pub struct DbSessionStore {
    db: Data<DatabaseConnection>,
}
impl DbSessionStore {
    pub fn new(db: Data<DatabaseConnection>) -> Self {
        Self { db }
    }
}

impl SessionStore for DbSessionStore {
    async fn load(&self, session_key: &SessionKey) -> Result<Option<SessionState>, LoadError> {
        use engelsystem_rs_db::session::SessionError as SE;

        let session = match load_session(&self.db, session_key.as_ref()).await {
            Ok(session) => session,
            Err(err) => match &err {
                SE::SessionNotFound => return Ok(None),
                SE::SessionDeserialize { .. } => Err(LoadError::Deserialization(err.into()))?,
                _ => {
                    error!("Error when loading session: {err}");
                    Err(LoadError::Other(err.into()))?
                }
            },
        };

        Ok(session)
    }

    async fn save(
        &self,
        session_state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, SaveError> {
        use engelsystem_rs_db::session::SessionError as SE;

        let session = match save_session(&self.db, session_state, ttl).await {
            Ok(session) => session,
            Err(err) => match &err {
                SE::SessionSerialize { .. } => Err(SaveError::Serialization(err.into()))?,
                _ => {
                    error!("Error when saving session: {err}");
                    Err(SaveError::Other(err.into()))?
                }
            },
        };

        let session_key: Result<SessionKey, _> = session.try_into();
        session_key.map_err(|e| SaveError::Other(e.into()))
    }

    async fn update(
        &self,
        session_key: SessionKey,
        session_state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, UpdateError> {
        use engelsystem_rs_db::session::SessionError as SE;

        if let Err(e) = update_session(&self.db, session_key.as_ref(), session_state, ttl).await {
            match e {
                SE::SessionSerialize { .. } => Err(UpdateError::Serialization(e.into()))?,
                _ => {
                    error!("Error when updating session: {e}");
                    Err(UpdateError::Other(e.into()))?
                }
            };
        }

        Ok(session_key)
    }

    async fn update_ttl(
        &self,
        session_key: &SessionKey,
        ttl: &Duration,
    ) -> Result<(), anyhow::Error> {
        match update_session_ttl(&self.db, session_key.as_ref(), ttl).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error when updating session TTL: {e}");
                Err(e.into())
            }
        }
    }

    async fn delete(&self, session_key: &SessionKey) -> Result<(), anyhow::Error> {
        match delete_session(&self.db, session_key.as_ref()).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error when deleting session: {e}");
                Err(e.into())
            }
        }
    }
}
