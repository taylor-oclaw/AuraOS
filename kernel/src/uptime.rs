extern crate alloc;
use alloc::string::String;
use alloc::format;

pub struct UptimeTracker {
    ticks: u64,
    ms_per_tick: u64,
}

impl UptimeTracker {
    pub fn new(ms_per_tick: u64) -> Self {
        Self { ticks: 0, ms_per_tick }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
    }

    pub fn uptime_ms(&self) -> u64 {
        self.ticks * self.ms_per_tick
    }

    pub fn uptime_seconds(&self) -> u64 {
        self.uptime_ms() / 1000
    }

    pub fn format_uptime(&self) -> String {
        let total = self.uptime_seconds();
        let days = total / 86400;
        let hours = (total % 86400) / 3600;
        let mins = (total % 3600) / 60;
        let secs = total % 60;

        if days > 0 {
            format!("{}d {}h {}m {}s", days, hours, mins, secs)
        } else if hours > 0 {
            format!("{}h {}m {}s", hours, mins, secs)
        } else if mins > 0 {
            format!("{}m {}s", mins, secs)
        } else {
            format!("{}s", secs)
        }
    }
}
