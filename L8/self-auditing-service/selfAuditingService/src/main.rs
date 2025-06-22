mod audit;

use axum::{
    routing::{post, get},
    Json, Router, response::{IntoResponse, Response},
    extract::Query,
};
use audit::{AuditLog, AuditEntry};
use chrono::Utc;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing_subscriber;

static AUDIT_LOG: Lazy<Mutex<AuditLog>> = Lazy::new(|| Mutex::new(AuditLog::new()));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/audit", post(log_action))
        .route("/audit_log", get(list_audit))
        .route("/compliance_report", get(compliance_report));

    println!("Self-Auditing Service running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// POST /audit  { "actor": "alice", "action": "login", "resource": "dashboard", "outcome": "success" }
async fn log_action(Json(mut entry): Json<AuditEntry>) -> impl IntoResponse {
    entry.timestamp = Utc::now();
    let mut log = AUDIT_LOG.lock().unwrap();
    log.log(entry.clone());
    Json(entry)
}

// GET /audit_log
async fn list_audit() -> impl IntoResponse {
    let log = AUDIT_LOG.lock().unwrap();
    Json(log.get())
}

// GET /compliance_report?format=csv|json
async fn compliance_report(Query(params): Query<std::collections::HashMap<String, String>>) -> Response {
    let log = AUDIT_LOG.lock().unwrap();
    if let Some(fmt) = params.get("format") {
        if fmt == "csv" {
            let body = log.as_csv();
            (
                axum::http::StatusCode::OK,
                [("Content-Type", "text/csv")],
                body,
            ).into_response()
        } else {
            Json(log.get()).into_response()
        }
    } else {
        Json(log.get()).into_response()
    }
}
