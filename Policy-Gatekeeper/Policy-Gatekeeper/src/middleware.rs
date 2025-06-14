use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::opa::check_policy;

pub async fn opa_gatekeeper(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let method = req.method().as_str();
    let path = req.uri().path();

    // For simplicity, assume user is extracted from header
    let user = req
        .headers()
        .get("x-user")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("anonymous");

    if check_policy(method, path, user).await {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
