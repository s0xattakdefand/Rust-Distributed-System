//! Tower service that routes to legacy or new backend based on path prefix.

use anyhow::{Result, anyhow};
use hyper::{Body, Response, Request, client::HttpConnector};
use tower::{Service, ServiceBuilder, service_fn};

pub fn strangler() -> impl Service<Request<Body>, Response=Response<Body>, Error=anyhow::Error> + Clone {
    ServiceBuilder::new().service(service_fn(router))
}

async fn router(req: Request<Body>) -> Result<Response<Body>> {
    let uri = req.uri().clone();
    if uri.path().starts_with("/v2") {
        Ok(Response::new(Body::from("Rust v2 response")))
    } else {
        // delegate to legacy host
        let client = hyper::Client::<HttpConnector, _>::new();
        let mut new_req = req;
        *new_req.uri_mut() = format!("http://legacy.local{}", uri).parse()?;
        client.request(new_req).await.map_err(|e| anyhow!(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test] async fn routes_new() {
        let svc = strangler();
        let req = Request::builder().uri("/v2/x").body(Body::empty()).unwrap();
        let resp = svc.oneshot(req).await.unwrap();
        assert_eq!(hyper::body::to_bytes(resp.into_body()).await.unwrap(), "Rust v2 response");
    }
}
