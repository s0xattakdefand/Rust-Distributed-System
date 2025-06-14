use hyper::{Request, Response, body::Body};
use hyper_util::{client::legacy::{connect::HttpConnector, Client}, rt::TokioExecutor};
use std::time::Duration;
use tower::{ServiceBuilder, ServiceExt};
use tower::util::{BoxService, service_fn};

pub fn build_breaker_stack(target_port: &'static str) -> BoxService<Request<Body>, Response<Body>, hyper::Error> {
    let svc = service_fn(move |_req: Request<Body>| {
        let uri = format!("http://127.0.0.1:{}/", target_port)
            .parse()
            .unwrap();

        let client = Client::builder(TokioExecutor::new()).build(HttpConnector::new());

        let req = Request::builder()
            .uri(uri)
            .body(Body::empty())
            .unwrap();

        client.request(req)
    });

    ServiceBuilder::new()
        .timeout(Duration::from_secs(2))
        .concurrency_limit(5)
        .service(svc)
        .boxed()
}
