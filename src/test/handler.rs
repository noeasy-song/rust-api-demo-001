use axum::{Json, extract::State};
use serde_json::json;
use sqlx::PgPool;

use crate::test::model::t_kline::Symbol;

pub async fn root() -> &'static str {
    "API Test Root Route"
}

pub async fn get_query(
    State(pool): State<PgPool>
) -> Json<serde_json::Value> {
    // SQL 쿼리 실행
    let rows: Vec<Symbol> = match sqlx::query_as!(
        Symbol,
        r#"
            SELECT DISTINCT symbol
            FROM bingx.t_kline
        "#
        )
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("DB query error: {}", e);
            return Json(json!({"error": "Failed to fetch data"}));
        }
    };

    // JSON 변환
    let json_rows: Vec<_> = rows
        .into_iter()
        .map(|row: Symbol| {
            let symbol: String = row.symbol;
            json!({ "symbol": symbol })
        })
        .collect();

    Json(json!({"data": json_rows}))
}

pub async fn get_query_file(
    State(pool): State<PgPool>
) -> Json<serde_json::Value> {
    // SQL 쿼리 실행
    let rows: Vec<Symbol> = match sqlx::query_file_as!(
            Symbol,
            "sql/test/select_symbols.sql"
    )
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("DB query error: {}", e);
            return Json(json!({"error": "Failed to fetch data"}));
        }
    };

    Json(json!({"data":rows}))
}