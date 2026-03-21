extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct SmartHomeTriggerTime {
    name: String,
    trigger_times: Vec<u32>,
}

impl SmartHomeTriggerTime {
    pub fn new(name: &str) -> Self {
        SmartHomeTriggerTime {
            name: String::from(name),
            trigger_times: Vec::new(),
        }
    }

    pub fn add_trigger_time(&mut self, time: u32) {
        self.trigger_times.push(time);
    }

    pub fn remove_trigger_time(&mut self, time: u32) -> bool {
        if let Some(index) = self.trigger_times.iter().position(|&t| t == time) {
            self.trigger_times.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_trigger_times(&self) -> &[u32] {
        &self.trigger_times
    }

    pub fn has_trigger_time(&self, time: u32) -> bool {
        self.trigger_times.contains(&time)
    }

    pub fn clear_trigger_times(&mut self) {
        self.trigger_times.clear();
    }
}
