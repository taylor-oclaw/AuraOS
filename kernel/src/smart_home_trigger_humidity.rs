extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct SmartHomeTriggerHumidity {
    min_humidity: u32,
    max_humidity: u32,
    triggers: Vec<String>,
}

impl SmartHomeTriggerHumidity {
    pub fn new(min_humidity: u32, max_humidity: u32) -> Self {
        SmartHomeTriggerHumidity {
            min_humidity,
            max_humidity,
            triggers: Vec::new(),
        }
    }

    pub fn add_trigger(&mut self, trigger: String) {
        if !self.triggers.contains(&trigger) {
            self.triggers.push(trigger);
        }
    }

    pub fn remove_trigger(&mut self, trigger: &str) -> bool {
        let pos = self.triggers.iter().position(|t| t == trigger);
        if let Some(index) = pos {
            self.triggers.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_within_range(&self, humidity: u32) -> bool {
        humidity >= self.min_humidity && humidity <= self.max_humidity
    }

    pub fn get_triggers(&self) -> &Vec<String> {
        &self.triggers
    }

    pub fn update_thresholds(&mut self, min_humidity: u32, max_humidity: u32) {
        self.min_humidity = min_humidity;
        self.max_humidity = max_humidity;
    }
}
