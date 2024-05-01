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

    



}