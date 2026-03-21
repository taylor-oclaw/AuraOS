extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() {}

struct SprinklerSystem {
    zones: Vec<String>,
    current_zone: usize,
    is_running: bool,
}

impl SprinklerSystem {
    pub fn new(zones: Vec<String>) -> Self {
        SprinklerSystem {
            zones,
            current_zone: 0,
            is_running: false,
        }
    }

    pub fn start(&mut self) {
        if !self.is_running && !self.zones.is_empty() {
            self.is_running = true;
            // Simulate starting sprinklers
        }
    }

    pub fn stop(&mut self) {
        if self.is_running {
            self.is_running = false;
            // Simulate stopping sprinklers
        }
    }

    pub fn next_zone(&mut self) -> Option<&str> {
        if self.is_running && !self.zones.is_empty() {
            self.current_zone = (self.current_zone + 1) % self.zones.len();
            Some(&self.zones[self.current_zone])
        } else {
            None
        }
    }

    pub fn current_zone_name(&self) -> Option<&str> {
        if !self.zones.is_empty() {
            Some(&self.zones[self.current_zone])
        } else {
            None
        }
    }

    pub fn add_zone(&mut self, zone: String) {
        self.zones.push(zone);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprinkler_system() {
        let mut system = SprinklerSystem::new(vec![
            String::from("Front Yard"),
            String::from("Back Yard"),
        ];

        assert!(!system.is_running);
        assert_eq!(system.current_zone_name(), Some("Front Yard"));

        system.start();
        assert!(system.is_running);

        system.next_zone();
        assert_eq!(system.current_zone_name(), Some("Back Yard"));

        system.stop();
        assert!(!system.is_running);

        system.add_zone(String::from("Garden"));
        assert_eq!(system.zones.len(), 3);
    }
}
