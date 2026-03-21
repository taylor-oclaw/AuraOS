extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextTimeAware {
    current_time: u64,
    events: Vec<(u64, String)>,
}

impl ContextTimeAware {
    pub fn new(initial_time: u64) -> Self {
        ContextTimeAware {
            current_time: initial_time,
            events: Vec::new(),
        }
    }

    pub fn set_current_time(&mut self, time: u64) {
        self.current_time = time;
    }

    pub fn get_current_time(&self) -> u64 {
        self.current_time
    }

    pub fn add_event(&mut self, event_time: u64, description: String) {
        if event_time >= self.current_time {
            self.events.push((event_time, description));
            self.events.sort_by_key(|&(time, _)| time);
        }
    }

    pub fn get_next_event(&self) -> Option<&(u64, String)> {
        self.events.get(0)
    }

    pub fn remove_next_event(&mut self) -> Option<(u64, String)> {
        if let Some((time, description)) = self.events.first() {
            if *time <= self.current_time {
                return self.events.remove(0);
            }
        }
        None
    }
}
