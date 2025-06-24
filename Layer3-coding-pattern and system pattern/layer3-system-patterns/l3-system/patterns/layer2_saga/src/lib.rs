//! Layer-2 Saga helper used by higher layers.

use anyhow::Result;

/// A saga step (action + compensation).
pub struct Step {
    pub action: Box<dyn Fn() -> Result<()> + Send + Sync + 'static>,
    pub compensate: Box<dyn Fn() + Send + Sync + 'static>,
}

impl Step {
    pub fn new<A, C>(action: A, compensate: C) -> Self
    where
        A: Fn() -> Result<()> + Send + Sync + 'static,
        C: Fn() + Send + Sync + 'static,
    {
        Self { action: Box::new(action), compensate: Box::new(compensate) }
    }
}

/// Run steps; on first failure roll back.
pub async fn run_saga(steps: Vec<Step>) -> Result<()> {
    let mut done: Vec<&Step> = Vec::new();
    for step in &steps {
        if let Err(e) = (step.action)() {
            for s in done.into_iter().rev() { (s.compensate)(); }
            return Err(e);
        }
        done.push(step);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test] async fn ok() {
        let s = Step::new(|| Ok(()), || {});
        assert!(run_saga(vec![s]).await.is_ok());
    }
}
