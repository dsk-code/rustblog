pub mod error;
pub mod repository;

pub use repository::users::UserRepository;

use error::Error;
use sqlx::PgPool;

pub struct DbConnector {
    pool: PgPool,
    secret: String,
}

impl DbConnector {
    pub fn new(pool: PgPool, secret: String) -> Self {
        Self { pool, secret }
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }

    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    pub async fn migration(&self) -> Result<(), Error> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(Error::MigrationError)?;
        Ok(())
    }
}

pub async fn init(pool: PgPool, secret: String) -> Result<DbConnector, Error> {
    let db = DbConnector::new(pool, secret);
    db.migration().await?;

    Ok(db)
}

