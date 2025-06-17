mod metrics;
mod drift;

use axum::{
    routing::get,
    Router,
    response::IntoResponse,
};
use std::sync::Mutex;
use std::time::{Instant, Duration};
use once_cell::sync::Lazy;
use metrics::ResponseTimeWindow;
use drift::is_drift;
use tracing::{info, warn};
use tracing_subscriber;

static BASELINE_LATENCY: Lazy<Mutex<f64>> = Lazy::new(|| Mutex::new(80.0)); // start with 80ms as historical
static WINDOW: Lazy<Mutex<ResponseTimeWindow>> = Lazy::new(|| Mutex::new(ResponseTimeWindow::new()));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/work", get(simulate_work))
        .route("/drift", get(drift_status));

    println!("Listening on http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// Simulates "work" endpoint; response time is tracked for drift
async fn simulate_work() -> impl IntoResponse {
    let mut rng = rand::random::<f64>();
    // Simulate "normal" latency (80ms ± 10ms, but sometimes 150-200ms to simulate a problem)
    let latency = if rng < 0.8 {
        70.0 + rand::random::<f64>() * 20.0 // 70-90ms
    } else {
        150.0 + rand::random::<f64>() * 50.0 // 150-200ms (simulate problem)
    };

    let now = Instant::now();
    tokio::time::sleep(Duration::from_millis(latency as u64)).await;
    let elapsed = now.elapsed().as_secs_f64() * 1000.0;

    WINDOW.lock().unwrap().record(elapsed);

    format!("Work simulated: latency {:.1}ms", elapsed)
}

// Endpoint to check for behavioral drift (for dashboard or alert manager)
async fn drift_status() -> impl IntoResponse {
    let window = WINDOW.lock().unwrap();
    let avg = window.avg_latency();

    let mut baseline = BASELINE_LATENCY.lock().unwrap();

    // Check drift: If window avg > baseline by 30% or more
    let drift = is_drift(*baseline, avg, 30.0);

    if drift {
        warn!(
            "Behavioral drift detected! Baseline: {:.1}ms, Current avg: {:.1}ms",
            *baseline, avg
        );
    } else {
        info!(
            "No drift. Baseline: {:.1}ms, Current avg: {:.1}ms",
            *baseline, avg
        );
    }

    // (Optional) Update baseline if we trust the new average over time (uncomment below)
    // *baseline = avg;

    if drift {
        format!("ALERT: Drift detected! Baseline: {:.1}ms, Current avg: {:.1}ms", *baseline, avg)
    } else {
        format!("Healthy. Baseline: {:.1}ms, Current avg: {:.1}ms", *baseline, avg)
    }
}
