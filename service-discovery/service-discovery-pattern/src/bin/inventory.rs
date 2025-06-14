use axum::{Router, routing::get, Json};
use service_discovery_pattern::service_registry::register::register_to_consul;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let service_name = "inventory";
    let service_address = "127.0.0.1";
    let service_port = 3001;

    // Register with Consul
    register_to_consul(service_name, service_address, service_port).await;

    // Create router
    let app = Router::new().route("/", get(handler));

    // Bind listener properly
    let addr = format!("{}:{}", service_address, service_port);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind TCP");

    println!("📦 Inventory service running at http://{}", addr);

    // Serve the app using axum::serve with TcpListener
    axum::serve(listener, app)
        .await
        .expect("Server error");
}

async fn handler() -> Json<&'static str> {
    Json("📦 Inventory: 42 T-shirts in stock")
}
