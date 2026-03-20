extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TimerSubsystem {
    timers: Vec<Timer>,
}

impl TimerSubsystem {
    pub fn new() -> Self {
        TimerSubsystem {
            timers: Vec::new(),
        }
    }

    pub fn add_timer(&mut self, name: String, duration: u64) {
        let timer = Timer { name, duration };
        self.timers.push(timer);
    }

    pub fn remove_timer(&mut self, name: &str) -> bool {
        if let Some(index) = self.timers.iter().position(|t| t.name == name) {
            self.timers.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_timer_duration(&self, name: &str) -> Option<u64> {
        self.timers.iter().find(|t| t.name == name).map(|t| t.duration)
    }

    pub fn list_timers(&self) -> Vec<String> {
        self.timers.iter().map(|t| t.name.clone()).collect()
    }

    pub fn tick(&mut self, elapsed: u64) {
        for timer in &mut self.timers {
            if timer.duration > 0 {
                timer.duration -= elapsed;
            }
        }
    }
}

struct Timer {
    name: String,
    duration: u64,
}
