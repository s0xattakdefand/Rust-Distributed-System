mod dao;

use axum::{
    routing::{post, get},
    Json, Router, response::IntoResponse, extract::{Path},
    http::StatusCode,
};
use dao::DAO;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing_subscriber;
use serde::Deserialize;

static DAO_STATE: Lazy<Mutex<DAO>> = Lazy::new(|| Mutex::new(DAO::new()));

#[derive(Debug, Deserialize)]
struct ProposeReq {
    proposer: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct VoteReq {
    voter: String,
    approve: bool,
}

// POST /propose
async fn propose(Json(req): Json<ProposeReq>) -> impl IntoResponse {
    let mut dao = DAO_STATE.lock().unwrap();
    let prop = dao.propose(req.proposer, req.description);
    Json(prop)
}

// POST /vote/:id
async fn vote(
    Path(id): Path<String>,
    Json(req): Json<VoteReq>,
) -> Result<Json<dao::Proposal>, (StatusCode, &'static str)> {
    let mut dao = DAO_STATE.lock().unwrap();
    if let Some(prop) = dao.vote(&id, req.voter, req.approve) {
        Ok(Json(prop))
    } else {
        Err((StatusCode::NOT_FOUND, "Proposal not found"))
    }
}

// POST /finalize/:id
async fn finalize(
    Path(id): Path<String>
) -> Result<Json<dao::Proposal>, (StatusCode, &'static str)> {
    let mut dao = DAO_STATE.lock().unwrap();
    if let Some(prop) = dao.finalize(&id) {
        Ok(Json(prop))
    } else {
        Err((StatusCode::NOT_FOUND, "Proposal not found"))
    }
}

// GET /proposals
async fn list_proposals() -> impl IntoResponse {
    let dao = DAO_STATE.lock().unwrap();
    Json(dao.list())
}

// GET /proposal/:id
async fn get_proposal(
    Path(id): Path<String>
) -> Result<Json<dao::Proposal>, (StatusCode, &'static str)> {
    let dao = DAO_STATE.lock().unwrap();
    if let Some(prop) = dao.get(&id) {
        Ok(Json(prop))
    } else {
        Err((StatusCode::NOT_FOUND, "Proposal not found"))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/propose", post(propose))
        .route("/vote/:id", post(vote))
        .route("/finalize/:id", post(finalize))
        .route("/proposals", get(list_proposals))
        .route("/proposal/:id", get(get_proposal));

    println!("Self-Governing/Community Infra running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
