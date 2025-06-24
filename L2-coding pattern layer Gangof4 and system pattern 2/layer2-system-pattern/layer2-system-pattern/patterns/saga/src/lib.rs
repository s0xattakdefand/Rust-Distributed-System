//! Minimal Saga executor.

use anyhow::Result;

/// One saga step: an `action` that may fail and a `compensate` function.
pub struct Step {
    pub action:     Box<dyn Fn() -> Result<()> + Send + Sync + 'static>,
    pub compensate: Box<dyn Fn() + Send + Sync + 'static>,
}

/// Execute steps. On first failure run compensations in reverse order.
pub async fn run_saga(steps: Vec<Step>) -> Result<()> {
    let mut completed: Vec<&Box<dyn Fn() + Send + Sync>> = Vec::new();

    for step in &steps {
        if let Err(e) = (step.action)() {
            for c in completed.into_iter().rev() { c(); }
            return Err(e);
        }
        completed.push(&step.compensate);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn compensates() {
        let log: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));

        let ok = Step {
            action: Box::new(|| Ok(())),
            compensate: Box::new(|| {}),
        };

        let log_clone = Arc::clone(&log);
        let fail = Step {
            action: Box::new(|| Err(anyhow::anyhow!("boom"))),
            compensate: Box::new(move || log_clone.lock().unwrap().push("undo")),
        };

        let res = tokio_test::block_on(run_saga(vec![ok, fail]));
        assert!(res.is_err());
        assert_eq!(*log.lock().unwrap(), vec!["undo"]);
    }
}
