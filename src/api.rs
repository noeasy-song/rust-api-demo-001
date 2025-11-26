use sqlx::PgPool;
use axum::{
    Router,
    routing::get
};
use crate::{
    test::handler as test_handler,
};

pub fn router(pool: PgPool) -> Router {
    let api_router = Router::new()
        // /api 최상위 Router
        .route("/", get(api_root_handler)
        )
        // /api/test
        .nest("/test", Router::new()
            .route("/", get(test_handler::root))
            .route("/get_query", get(test_handler::get_query))
            .route("/get_query_file", get(test_handler::get_query_file))
        )
        .with_state(pool);

    // 최상위 라우터
    Router::new()
        .route("/", get(root_handler))
        .nest("/api", api_router)
}

async fn root_handler() -> &'static str {
    "Root Route"
}

async fn api_root_handler() -> &'static str {
    "API Root Route"
}