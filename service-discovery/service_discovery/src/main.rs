mod discovery;
mod service;

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use discovery::{ServiceRegistry, SharedRegistry};
use service::ServiceInfo;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let registry: SharedRegistry = Arc::new(Mutex::new(ServiceRegistry::new()));

    let app = Router::new()
        .route("/register/:name/:address", post({
            let registry = registry.clone();
            move |Path((name, address)): Path<(String, String)>| {
                let registry = registry.clone();
                async move {
                    let service = ServiceInfo { name: name.clone(), address: address.clone() };
                    registry.lock().await.register(service);
                    Json(format!("✅ '{}' registered at {}", name, address))
                }
            }
        }))
        .route("/discover/:name", get({
            let registry = registry.clone();
            move |Path(name): Path<String>| {
                let registry = registry.clone();
                async move {
                    let reg = registry.lock().await;
                    match reg.discover(&name) {
                        Some(info) => Json(format!("🔍 '{}' found at {:?}", name, info)),
                        None => Json(format!("❌ '{}' not found.", name)),
                    }
                }
            }
        }));

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    println!("🚀 Service Discovery running at http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
