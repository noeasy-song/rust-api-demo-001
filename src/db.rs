use dotenvy::{
    dotenv,
    var
};
use sqlx::{
    PgPool,
    postgres::PgPoolOptions
};

pub async fn get_pool() -> PgPool {
    dotenv().ok();

    let pg_user: String = var("PG_DATABASE_USER").expect("PG_DATABASE_USER not set");
    let pg_password: String = var("PG_DATABASE_PASSWORD").expect("PG_DATABASE_PASSWORD not set");
    let pg_host: String = var("PG_DATABASE_HOST").expect("PG_DATABASE_HOST not set");
    let pg_port: String = var("PG_DATABASE_PORT").expect("PG_DATABASE_PORT not set");
    let pg_db: String = var("PG_DATABASE_DB").expect("PG_DATABASE_DB not set");
    
    // let url: String = var("PG_DATABASE_URL").expect("DATABASE_URL not set");
    let url: String = format!("postgres://{}:{}@{}:{}/{}",
        pg_user,
        pg_password,
        pg_host,
        pg_port,
        pg_db
    );

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("DB connection failed")
}