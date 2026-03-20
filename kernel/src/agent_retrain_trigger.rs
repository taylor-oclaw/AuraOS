extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn agent_retrain_trigger_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn agent_retrain_trigger_exit() {
    // Cleanup logic for the module
}

pub struct AgentRetrainTrigger {
    triggers: Vec<String>,
    active: bool,
}

impl AgentRetrainTrigger {
    pub fn new() -> Self {
        AgentRetrainTrigger {
            triggers: Vec::new(),
            active: false,
        }
    }

    pub fn add_trigger(&mut self, trigger: String) {
        self.triggers.push(trigger);
    }

    pub fn remove_trigger(&mut self, trigger: &str) -> bool {
        if let Some(index) = self.triggers.iter().position(|t| t == trigger) {
            self.triggers.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_triggers(&self) -> Vec<String> {
        self.triggers.clone()
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_retrain_trigger() {
        let mut trigger = AgentRetrainTrigger::new();
        assert!(!trigger.is_active());

        trigger.add_trigger(String::from("trigger1"));
        trigger.add_trigger(String::from("trigger2"));

        assert_eq!(trigger.list_triggers(), vec![String::from("trigger1"), String::from("trigger2")]);

        assert!(trigger.remove_trigger("trigger1"));
        assert!(!trigger.remove_trigger("trigger3"));

        assert_eq!(trigger.list_triggers(), vec![String::from("trigger2")]);

        trigger.activate();
        assert!(trigger.is_active());

        trigger.deactivate();
        assert!(!trigger.is_active());
    }
}
