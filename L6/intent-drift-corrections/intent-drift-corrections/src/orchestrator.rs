pub enum Correction {
    None,
    ScaleUp,
    Reroute,
    Restart,
}

pub fn detect_and_correct(desired: f64, current: f64) -> (bool, Correction) {
    // If average latency > desired intent by 30%, trigger correction
    if current > desired * 1.3 {
        if current > desired * 2.0 {
            (true, Correction::Reroute)
        } else {
            (true, Correction::ScaleUp)
        }
    } else {
        (false, Correction::None)
    }
}
