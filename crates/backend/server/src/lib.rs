use auth::{self, key_init};

pub mod error;
use error::Error;
use shuttle_secrets::SecretStore;

pub fn init(secrets: SecretStore) -> Result<(), Error>{
    let auth_secret = secrets
        .get("AUTH_SECRET")
        .ok_or(Error::NotFoundSecrets("AUTH_SECRET".into()))?;
    
    key_init(&auth_secret.as_ref()).map_err(|e| Error::AuthError(e))
}