use dotenvy::{
    dotenv,
    var
};
use rust_api_demo_001::app_router;

use tokio::net::TcpListener;
use axum::{serve, Router};

#[tokio::main]
// async fn main() {
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let app_url: String = var("APP_URL").expect("APP_URL not set");
    let app_port: String = var("APP_PORT").expect("APP_PORT not set");
    let addr: String = format!("{}:{}", app_url, app_port);

    let make_service: Router = app_router().await;
    let listener: TcpListener = TcpListener::bind(addr).await.unwrap();

    serve(listener, make_service).await.unwrap();

    Ok(())
}