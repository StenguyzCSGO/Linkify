use std::time::Duration;

pub struct DevResponse {
    message: String,
    timings: Vec<(&'static str, Duration)>,
}

impl DevResponse {
    pub fn new(message: String) -> Self {
        Self {
            message,
            timings: Vec::new(),
        }
    }

    pub fn add_timing(mut self, label: &'static str, duration: Duration) -> Self {
        self.timings.push((label, duration));
        self
    }

    pub fn build(self) -> String {
        if is_dev_mode() {
            let timing_info = if !self.timings.is_empty() {
                let timings_str = self
                    .timings
                    .iter()
                    .map(|(label, duration)| format!("{}: {:.2}s", label, duration.as_secs_f64()))
                    .collect::<Vec<_>>()
                    .join(", ");
                
                let total = self.timings.iter().map(|(_, d)| d.as_secs_f64()).sum::<f64>();
                format!("\n\n`Execution time: {:.2}s ({})`", total, timings_str)
            } else {
                String::new()
            };
            
            format!("{}{}", self.message, timing_info)
        } else {
            self.message
        }
    }
}

pub fn is_dev_mode() -> bool {
    std::env::var("GUILD_ID").is_ok()
}
