use std::sync::Arc;

// use auth;
use server;

use shuttle_runtime::SecretStore;
use axum::Router;
use sqlx::PgPool;
use anyhow::Context;


// async fn hello_world() -> &'static str {
//     "Hello, world!"
// }

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "{secrets.DATABASE_URL}"
    )] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // let router = Router::new().route("/", get(hello_world));
    let state = server::init(secrets, pool).await.context("failed to init")?;
    let state = Arc::new(state);
    
    let app = Router::new()
        // .route("/", get(hello_world))
        .merge(server::router::static_file::static_roouter())
        .nest("/auth", server::router::auth::router(state.clone()));
    
    Ok(app.into())
}