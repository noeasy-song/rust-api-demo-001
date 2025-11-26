use axum::Router;
use sqlx::PgPool;

pub mod api;
pub mod db;

pub mod test;

pub async fn db_connect() -> PgPool {
    db::get_pool().await
}

pub async fn app_router() -> Router {
    let pool: PgPool = db_connect().await;
    api::router(pool)
}