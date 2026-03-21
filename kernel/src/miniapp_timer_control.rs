extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MiniAppTimerControl {
    timers: Vec<Timer>,
}

impl MiniAppTimerControl {
    pub fn new() -> Self {
        MiniAppTimerControl {
            timers: Vec::new(),
        }
    }

    pub fn add_timer(&mut self, name: String, duration: u64) {
        let timer = Timer { name, duration };
        self.timers.push(timer);
    }

    pub fn remove_timer(&mut self, index: usize) -> Option<Timer> {
        if index < self.timers.len() {
            Some(self.timers.remove(index))
        } else {
            None
        }
    }

    pub fn get_timer(&self, index: usize) -> Option<&Timer> {
        self.timers.get(index)
    }

    pub fn list_timers(&self) -> Vec<String> {
        self.timers.iter().map(|t| t.name.clone()).collect()
    }

    pub fn update_timer_duration(&mut self, index: usize, new_duration: u64) -> bool {
        if let Some(timer) = self.timers.get_mut(index) {
            timer.duration = new_duration;
            true
        } else {
            false
        }
    }
}

struct Timer {
    name: String,
    duration: u64,
}
