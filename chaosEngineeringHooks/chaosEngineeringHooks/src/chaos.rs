use rand::Rng;
use tracing::warn;

/// Defines chaos injection probability and simulates random failures.
pub fn inject_fault(probability: f64) -> Result<(), &'static str> {
    let mut rng = rand::thread_rng();
    let roll: f64 = rng.gen();
    
    if roll < probability {
        warn!("Chaos hook activated! Injecting fault...");
        Err("Injected Chaos Fault")
    } else {
        Ok(())
    }
}
