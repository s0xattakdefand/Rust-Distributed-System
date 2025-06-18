mod knowledge;
mod router;

use axum::{
    extract::{Query, Json},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tracing::{info, warn};
use tracing_subscriber;
use knowledge::KnowledgeBase;
use chrono::Utc;

static KNOWLEDGE: Lazy<Mutex<KnowledgeBase>> = Lazy::new(|| Mutex::new(KnowledgeBase::new()));

#[derive(Debug, Deserialize)]
struct RequestRegion {}

#[derive(Debug, Serialize)]
struct RoutingResponse {
    routed_to: String,
    knowledge_snapshot: Vec<knowledge::Incident>,
}

#[derive(Debug, Deserialize)]
struct IncidentReport {
    region: String,
    reason: String,
    cooldown_secs: i64,
}

// GET /route (returns the best available region, avoiding recent incidents)
async fn route_handler() -> impl IntoResponse {
    let mut kb = KNOWLEDGE.lock().unwrap();
    kb.clean_expired();
    let chosen = router::choose_region(&kb);
    let snapshot = kb.snapshot();
    Json(RoutingResponse {
        routed_to: chosen,
        knowledge_snapshot: snapshot,
    })
}

// POST /report_incident { "region": "asia", "reason": "db down", "cooldown_secs": 60 }
async fn report_incident(Json(inc): Json<IncidentReport>) -> impl IntoResponse {
    let mut kb = KNOWLEDGE.lock().unwrap();
    kb.remember_failure(&inc.region, &inc.reason, inc.cooldown_secs);
    warn!(
        "Incident reported: region={} reason={} cooldown={}s",
        inc.region, inc.reason, inc.cooldown_secs
    );
    Json(serde_json::json!({"status":"recorded"}))
}

// GET /knowledge
async fn knowledge_status() -> impl IntoResponse {
    let kb = KNOWLEDGE.lock().unwrap();
    Json(kb.snapshot())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/route", get(route_handler))
        .route("/report_incident", post(report_incident))
        .route("/knowledge", get(knowledge_status));

    println!("Listening on http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
