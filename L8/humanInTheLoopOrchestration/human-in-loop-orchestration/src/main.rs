use actix_web::{Either, get, web, HttpResponse, Responder, App, HttpServer};
use serde::Serialize;

/// The payload we return when the decision is automatically approved.
#[derive(Serialize)]
struct Decision {
    accepted: bool,
    note: String,
}

/// GET /decide – returns JSON when approved, 202 text when pending.
/// In real life you’d compute `approved` from business logic.
#[get("/decide")]
async fn decide() -> Either<web::Json<Decision>, HttpResponse> {
    // Dummy logic – hard-coded approval for the demo
    let approved = true;

    if approved {
        let body = Decision {
            accepted: true,
            note: "Automatically approved ✓".into(),
        };
        Either::Left(web::Json(body))
    } else {
        Either::Right(
            HttpResponse::Accepted()
                .body("Pending manual review ⏳"),
        )
    }
}

/// Spin up the Actix server on 0.0.0.0:8080
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Listening on http://localhost:8080");
    HttpServer::new(|| App::new().service(decide))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
