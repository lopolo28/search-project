mod api_tests;

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

#[derive(Debug)]
struct EnvValues {
    ip: String,
    port: String,
    api_key: String,
    search_engine_id: String,
}

#[tokio::main]
async fn main() {
    let env_vars = match load_env_vars() {
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
        Ok(env) => env,
    };

    let state = AppState {
        google_api_key: env_vars.api_key,
        google_search_engine_id: env_vars.search_engine_id,
    };

    let app = Router::new()
        .route("/api/search", post(search))
        .route("/", get(index))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", env_vars.ip, env_vars.port))
        .await
        .unwrap();

    println!("Server hosted at {}:{}", env_vars.ip, env_vars.port);
    axum::serve(listener, app).await.unwrap();
}

fn load_env_vars() -> Result<EnvValues, String> {
    dotenv::dotenv().ok();

    let ip = env::var("IP").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let api_key = env::var("GOOGLE_API_KEY");
    let search_engine_id = env::var("GOOGLE_SEARCH_ENGINE_ID");

    if api_key.is_err() {
        return Err("GOOGLE_API_KEY must be set".to_string());
    }

    if search_engine_id.is_err() {
        return Err("GOOGLE_SEARCH_ENGINE_ID must be set".to_string());
    }

    Ok(EnvValues {
        ip,
        port,
        api_key: api_key.unwrap(),
        search_engine_id: search_engine_id.unwrap(),
    })
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
