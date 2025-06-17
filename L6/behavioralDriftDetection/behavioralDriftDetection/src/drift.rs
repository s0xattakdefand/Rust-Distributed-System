pub fn is_drift(baseline: f64, current: f64, threshold_percent: f64) -> bool {
    if baseline == 0.0 {
        return false; // not enough data
    }
    let percent_diff = ((current - baseline) / baseline).abs() * 100.0;
    percent_diff > threshold_percent
}
