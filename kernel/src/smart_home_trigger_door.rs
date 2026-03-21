extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

#[repr(C)]
pub struct SmartHomeTriggerDoor {
    name: String,
    is_open: bool,
    triggers: Vec<String>,
}

impl SmartHomeTriggerDoor {
    pub fn new(name: &str) -> Self {
        SmartHomeTriggerDoor {
            name: String::from(name),
            is_open: false,
            triggers: Vec::new(),
        }
    }

    pub fn open(&mut self) {
        if !self.is_open {
            self.is_open = true;
            // Simulate opening the door
        } else {
        }
    }

    pub fn close(&mut self) {
        if self.is_open {
            self.is_open = false;
            // Simulate closing the door
        } else {
        }
    }

    pub fn add_trigger(&mut self, trigger: &str) {
        self.triggers.push(String::from(trigger));
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_home_trigger_door() {
        let mut door = SmartHomeTriggerDoor::new("Front Door");

        assert!(!door.is_open);
        door.open();
        assert!(door.is_open);

        door.close();
        assert!(!door.is_open);

        door.add_trigger("Motion Detected");
        assert_eq!(door.list_triggers(), vec![String::from("Motion Detected")]);

        door.add_trigger("Voice Command");
        assert_eq!(
            door.list_triggers(),
            vec![String::from("Motion Detected"), String::from("Voice Command")]
        ;

        assert!(door.remove_trigger("Motion Detected"));
        assert_eq!(door.list_triggers(), vec![String::from("Voice Command")]);

        assert!(!door.remove_trigger("Non-existent Trigger"));
    }
}
