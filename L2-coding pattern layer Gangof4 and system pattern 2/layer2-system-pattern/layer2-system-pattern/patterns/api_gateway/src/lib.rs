//! Mini API-Gateway for **Axum 0.7**
//! – REST route, GraphQL stub, simple cookie check.
//
// Rate-limit is temporarily removed; Tower’s `RateLimitLayer` service is not
// `Clone`, which Axum’s `Router::layer` requires.  Add a custom middleware
// later if you need throttling.

use axum::{
    extract::State,
    routing::get,
    Json, Router, serve,
};
use serde_json::json;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_cookies::{CookieManagerLayer, Cookies};

#[derive(Clone, Default)]
pub struct AppState;

pub async fn start() -> anyhow::Result<()> {
    let state = Arc::new(AppState);

    let app = Router::new()
        .route("/api/hello", get(rest_hello))
        .route("/graphql",   get(graphql_stub))
        .route("/greet",     get(composite))
        .layer(CookieManagerLayer::new())
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let listener = TcpListener::bind(addr).await?;
    println!("API Gateway listening on {addr}");
    serve(listener, app).await?;
    Ok(())
}

// -- handlers --------------------------------------------------------------

async fn rest_hello(State(_): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(json!({ "msg": "Hello from REST" }))
}

async fn graphql_stub() -> Json<serde_json::Value> {
    Json(json!({ "data": { "hello": "world" } }))
}

async fn composite(cookies: Cookies) -> Json<serde_json::Value> {
    let user = cookies.get("jwt").map(|c| c.value().to_owned()).unwrap_or_else(|| "anon".into());
    Json(json!({ "combined": [format!("REST hi {user}"), "GraphQL hi"] }))
}

// -- tests ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn hello_ok() {
        let resp = rest_hello(State(Arc::default())).await;
        assert_eq!(resp.0["msg"], "Hello from REST");
    }
}
