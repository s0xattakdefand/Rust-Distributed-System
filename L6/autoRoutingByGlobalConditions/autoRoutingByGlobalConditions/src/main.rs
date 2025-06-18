mod global_state;
mod router;

use axum::{
    extract::{Query, State},
    routing::{get, post},
    Router,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use global_state::{GlobalConditions, RegionStatus};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing::{info, warn};
use tracing_subscriber;

static GLOBAL: Lazy<Mutex<GlobalConditions>> = Lazy::new(|| Mutex::new(GlobalConditions::default()));

#[derive(Debug, Deserialize)]
struct RouteRequest {
    user_region: String,
}

#[derive(Debug, Serialize)]
struct RouteResponse {
    routed_to: String,
    global_snapshot: GlobalConditions,
}

#[derive(Debug, Deserialize)]
struct UpdateRegion {
    region: String,
    status: RegionStatus,
}

// GET: /route?user_region=asia
async fn route_handler(Query(q): Query<RouteRequest>) -> impl IntoResponse {
    let global = GLOBAL.lock().unwrap().clone();
    let route = router::pick_route(&q.user_region, &global);
    Json(RouteResponse {
        routed_to: route,
        global_snapshot: global,
    })
}

// POST: /update_region { "region": "asia", "status": "UnderAttack" }
async fn update_region(Json(upd): Json<UpdateRegion>) -> impl IntoResponse {
    let mut global = GLOBAL.lock().unwrap();
    global.region_status.insert(upd.region.clone(), upd.status.clone());
    info!(
        "Region '{}' status updated to {:?}",
        upd.region, upd.status
    );
    Json(global.clone())
}

// GET: /global_status
async fn global_status() -> impl IntoResponse {
    let global = GLOBAL.lock().unwrap().clone();
    Json(global)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/route", get(route_handler))
        .route("/update_region", post(update_region))
        .route("/global_status", get(global_status));

    println!("Listening on http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
