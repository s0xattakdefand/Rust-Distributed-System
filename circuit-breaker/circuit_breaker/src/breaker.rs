use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum State {
    Closed,
    Open(Instant),
    HalfOpen,
}

pub struct CircuitBreaker {
    state: State,
    failure_count: u32,
    threshold: u32,
    retry_timeout: Duration,
}

impl CircuitBreaker {
    pub fn new(threshold: u32, retry_timeout_secs: u64) -> Self {
        Self {
            state: State::Closed,
            failure_count: 0,
            threshold,
            retry_timeout: Duration::from_secs(retry_timeout_secs),
        }
    }

    pub fn can_call(&mut self) -> bool {
        match &self.state {
            State::Closed => true,
            State::Open(since) => {
                if since.elapsed() >= self.retry_timeout {
                    self.state = State::HalfOpen;
                    true
                } else {
                    false
                }
            }
            State::HalfOpen => true,
        }
    }

    pub fn record_success(&mut self) {
        self.state = State::Closed;
        self.failure_count = 0;
    }

    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        if self.failure_count >= self.threshold {
            self.state = State::Open(Instant::now());
        }
    }
}
