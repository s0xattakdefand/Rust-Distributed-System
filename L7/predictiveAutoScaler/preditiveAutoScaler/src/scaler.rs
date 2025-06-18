use std::collections::VecDeque;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ScalingState {
    pub recent_traffic: Vec<u32>,
    pub predicted_next: u32,
    pub scale_level: u32,
    pub last_action: String,
}

pub struct PredictiveScaler {
    pub window: VecDeque<u32>,
    pub window_size: usize,
    pub scale_level: u32,
    pub last_action: String,
}

impl PredictiveScaler {
    pub fn new(window_size: usize) -> Self {
        Self {
            window: VecDeque::with_capacity(window_size),
            window_size,
            scale_level: 1,
            last_action: "init".to_string(),
        }
    }

    /// Simulate new traffic data, update prediction, and scale if needed
    pub fn add_traffic(&mut self, traffic: u32) {
        if self.window.len() == self.window_size {
            self.window.pop_front();
        }
        self.window.push_back(traffic);

        let prediction = self.predict_next();

        // Predictive scaling logic: if the predicted traffic > 1.5x average, scale up
        let avg: f32 = if self.window.len() > 0 {
            self.window.iter().sum::<u32>() as f32 / self.window.len() as f32
        } else { 0.0 };

        if prediction as f32 > avg * 1.5 {
            self.scale_level += 1;
            self.last_action = format!("scale_up (to {})", self.scale_level);
        } else if ((prediction as f32) < avg * 0.6) && (self.scale_level > 1) {
            self.scale_level -= 1;
            self.last_action = format!("scale_down (to {})", self.scale_level);
        } else {
            self.last_action = "hold".to_string();
        }
    }

    pub fn predict_next(&self) -> u32 {
        // Simple: If last two points are both up, predict up, else average
        if self.window.len() >= 2 {
            let w = &self.window;
            let last = w[w.len()-1];
            let prev = w[w.len()-2];
            if last > prev {
                // Extrapolate up!
                let delta = last - prev;
                return last + delta;
            }
        }
        // Fallback: simple moving average
        if self.window.is_empty() {
            0
        } else {
            self.window.iter().sum::<u32>() / self.window.len() as u32
        }
    }

    pub fn state(&self) -> ScalingState {
        ScalingState {
            recent_traffic: self.window.iter().cloned().collect(),
            predicted_next: self.predict_next(),
            scale_level: self.scale_level,
            last_action: self.last_action.clone(),
        }
    }
}
