use auth;
use db::DbConnector;

pub mod error;
pub mod router;

use error::Error;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use std::sync::Arc;

pub struct State {
    db: Arc<DbConnector>
}

impl State {
    pub fn new(db: DbConnector) -> Self {
        Self{ db: Arc::new(db) }
    }

    pub fn db(&self) -> Arc<DbConnector> {
        self.db.clone()
    }
}

pub async fn init(secrets: SecretStore, pool: PgPool) -> Result<State, Error>{
    let auth_secret = secrets
        .get("AUTH_SECRET")
        .ok_or(Error::NotFoundSecrets("AUTH_SECRET".into()))?;
    auth::key_init(&auth_secret.as_ref())?;

    let db_secret = secrets
        .get("DB_SECRET")
        .ok_or(Error::NotFoundSecrets("DB_SECRET".into()))?;
    let db = State::new(DbConnector::new(pool, db_secret));
    Ok(db)

}