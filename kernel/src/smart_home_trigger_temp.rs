extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SmartHomeTriggerTemp {
    temperature_threshold: i32,
    current_temperature: i32,
    triggers: Vec<String>,
}

impl SmartHomeTriggerTemp {
    pub fn new(temperature_threshold: i32, initial_temperature: i32) -> Self {
        SmartHomeTriggerTemp {
            temperature_threshold,
            current_temperature: initial_temperature,
            triggers: Vec::new(),
        }
    }

    pub fn set_temperature(&mut self, temperature: i32) {
        self.current_temperature = temperature;
    }

    pub fn get_temperature(&self) -> i32 {
        self.current_temperature
    }

    pub fn add_trigger(&mut self, trigger: String) {
        self.triggers.push(trigger);
    }

    pub fn remove_trigger(&mut self, index: usize) -> Option<String> {
        if index < self.triggers.len() {
            Some(self.triggers.remove(index))
        } else {
            None
        }
    }

    pub fn check_triggers(&self) -> Vec<&String> {
        let mut active_triggers = Vec::new();
        for trigger in &self.triggers {
            if self.current_temperature >= self.temperature_threshold {
                active_triggers.push(trigger);
            }
        }
        active_triggers
    }
}
