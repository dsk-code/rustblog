pub mod users;

#[cfg(test)]
pub mod tests_utils {
    use crate::DbConnector;
    use sqlx::PgPool;

    pub const TEST_SECRET: &str = "secret";

    pub async fn test_db_conector(pool: PgPool) -> DbConnector {
        DbConnector::new(pool, TEST_SECRET.into())
    }
}
