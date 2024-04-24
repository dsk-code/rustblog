use auth::error;

use thiserror::Error;


#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    AuthError(#[from] error::AuthError),
    #[error("Not Found secrets: {0}")]
    NotFoundSecrets(String),
}