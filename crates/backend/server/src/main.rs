// use auth;
use server;

use shuttle_runtime::SecretStore;
use axum::{routing::get, Router};
use sqlx::PgPool;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] _pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));
    let state = server::init(secrets);
    match state {
        Ok(_) => println!("key is ok"),
        Err(e) => eprintln!("ERROR: {}", e),
    }
    
    Ok(router.into())
}