//! Backend-for-Frontend stub: merges two mock micro-services.

use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct UiView { products: Vec<&'static str>, cart_total: f32 }

#[get("/home")]
async fn home() -> impl Responder {
    // fake downstream calls
    let products = vec!["Book", "Pen"];
    let cart_total = 12.5;
    web::Json(UiView { products, cart_total })
}

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home)).bind(("0.0.0.0", 7000))?.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[actix_rt::test]   // now resolves because actix-rt dep is present
    async fn view_serialises() {
        let v = UiView { products: vec![], cart_total: 0.0 };
        assert_eq!(serde_json::to_string(&v).unwrap(),
                   r#"{"products":[],"cart_total":0.0}"#);
    }
}

