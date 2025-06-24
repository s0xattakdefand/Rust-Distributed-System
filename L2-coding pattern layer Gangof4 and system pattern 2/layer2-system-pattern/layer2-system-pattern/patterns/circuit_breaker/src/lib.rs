//! Tower Service–based Circuit Breaker (Send + 'static safe).

use anyhow::anyhow;
use futures_util::future::BoxFuture;
use std::{
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::{Duration, Instant},
};
use tower::{Layer, Service};

/// Builder for a circuit breaker layer.
#[derive(Debug, Clone)]
pub struct BreakerLayer {
    pub max_failures: usize,
    pub window:       Duration,
}

impl BreakerLayer {
    pub fn new(max_failures: usize, window: Duration) -> Self {
        Self { max_failures, window }
    }
}

impl<S> Layer<S> for BreakerLayer {
    type Service = Breaker<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Breaker {
            inner,
            cfg: self.clone(),
            state: Arc::new(Mutex::new(State::Closed { fails: 0, since: Instant::now() })),
        }
    }
}

#[derive(Debug)]
enum State {
    Closed { fails: usize, since: Instant },
    Open   { until: Instant },
}

#[derive(Clone)]
pub struct Breaker<S> {
    inner: S,
    cfg:   BreakerLayer,
    state: Arc<Mutex<State>>,
}

impl<Req, S, Res> Service<Req> for Breaker<S>
where
    Req: Send + 'static,
    Res: Send + 'static,
    S: Service<Req, Response = Res, Error = anyhow::Error> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Res;
    type Error    = anyhow::Error;
    type Future   = BoxFuture<'static, Result<Res, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        futures_util::ready!(self.inner.poll_ready(cx))?;
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let mut inner = self.inner.clone();
        let state     = self.state.clone();
        let cfg       = self.cfg.clone();
        let now       = Instant::now();

        Box::pin(async move {
            // short-circuit if open
            {
                let s = state.lock().unwrap();
                if let State::Open { until } = *s {
                    if until > now {
                        return Err(anyhow!("circuit open"));
                    }
                }
            }

            match inner.call(req).await {
                Ok(res) => {
                    *state.lock().unwrap() = State::Closed { fails: 0, since: now };
                    Ok(res)
                }
                Err(e) => {
                    let mut st = state.lock().unwrap();
                    match &mut *st {
                        State::Closed { fails, since } => {
                            *fails += 1;
                            if *fails >= cfg.max_failures && since.elapsed() < cfg.window {
                                *st = State::Open { until: now + cfg.window };
                            }
                        }
                        State::Open { .. } => {}
                    }
                    Err(e)
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::service_fn;

    #[tokio::test]
    async fn trips_after_two_failures() {
        // ---- ⬇︎ THIS LINE CHANGED ⬇︎ -------------------------------------
        let failing = service_fn(|_req: ()| async {
            Err::<(), anyhow::Error>(anyhow!("boom"))
        });
        // -----------------------------------------------------------------

        let layer   = BreakerLayer::new(2, std::time::Duration::from_secs(1));
        let mut svc = layer.layer(failing);

        assert!(svc.call(()).await.is_err());
        assert!(svc.call(()).await.is_err());
        let err = svc.call(()).await.unwrap_err();
        assert_eq!(err.to_string(), "circuit open");
    }
}

