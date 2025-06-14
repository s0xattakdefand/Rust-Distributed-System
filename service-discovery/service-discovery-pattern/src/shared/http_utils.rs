use axum::Router;
use tokio::net::TcpListener;

pub async fn start_service(name: &str, port: u16, handler: Router) {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind TCP");
    println!("🚀 Starting {} on {}", name, addr);

    axum::serve(listener, handler)
        .await
        .expect("Failed to start server");
}
