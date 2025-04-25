use std::env;

use axum::{
    Json, Router,
    extract::State,
    response::{Html, Result},
    routing::{get, post},
};

use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}

#[derive(Clone)]
pub struct AppState {
    google_api_key: String,
    google_search_engine_id: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); // Load environment variables

    let ip = env::var("IP").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let api_key = env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");
    let search_engine_id =
        env::var("GOOGLE_SEARCH_ENGINE_ID").expect("GOOGLE_SEARCH_ENGINE_ID must be set");

    let state = AppState {
        google_api_key: api_key,
        google_search_engine_id: search_engine_id,
    };

    let app = Router::new()
        .route("/api/search", post(search))
        .route("/", get(index))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", ip, port))
        .await
        .unwrap();

    println!("Server hosted at {}:{}", ip, port);
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Result<Html<String>> {
    let html = include_str!("./index.html");
    Ok(Html(html.to_string()))
}
async fn make_google_search(
    query: &str,
    api_key: &str,
    search_engine_id: &str,
) -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "https://www.googleapis.com/customsearch/v1";

    let response = client
        .get(url)
        .query(&[("key", api_key), ("cx", search_engine_id), ("q", query)])
        .send()
        .await?;

    response.json().await
}
async fn search(
    State(state): State<AppState>,
    Json(search_request): Json<SearchQuery>,
) -> Result<Json<serde_json::Value>> {
    let query = search_request.query;

    match make_google_search(
        &query,
        &state.google_api_key,
        &state.google_search_engine_id,
    )
    .await
    {
        Ok(results) => Ok(Json(
            json!({ "code": 200, "success": true, "payload": results }),
        )),
        Err(e) => {
            eprintln!("Error from requesting query: {}", e);
            Ok(Json(
                json!({ "code": 500, "success": false, "payload": json!({}) }),
            ))
        }
    }
}
