extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct FanController {
    fans: Vec<Fan>,
}

impl FanController {
    pub fn new() -> Self {
        FanController { fans: Vec::new() }
    }

    pub fn add_fan(&mut self, fan: Fan) {
        self.fans.push(fan);
    }

    pub fn remove_fan(&mut self, index: usize) -> Option<Fan> {
        if index < self.fans.len() {
            Some(self.fans.remove(index))
        } else {
            None
        }
    }

    pub fn get_fan_status(&self, index: usize) -> Option<&FanStatus> {
        self.fans.get(index).map(|fan| &fan.status)
    }

    pub fn set_fan_speed(&mut self, index: usize, speed: u8) -> bool {
        if let Some(fan) = self.fans.get_mut(index) {
            fan.speed = speed;
            true
        } else {
            false
        }
    }

    pub fn get_all_fan_statuses(&self) -> Vec<&FanStatus> {
        self.fans.iter().map(|fan| &fan.status).collect()
    }
}

struct Fan {
    speed: u8,
    status: FanStatus,
}

#[derive(Debug)]
enum FanStatus {
    On,
    Off,
    Error(String),
}

impl Fan {
    fn new(speed: u8) -> Self {
        Fan {
            speed,
            status: FanStatus::Off,
        }
    }

    fn turn_on(&mut self) {
        self.status = FanStatus::On;
    }

    fn turn_off(&mut self) {
        self.status = FanStatus::Off;
    }

    fn set_error(&mut self, error_message: String) {
        self.status = FanStatus::Error(error_message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fan_controller() {
        let mut controller = FanController::new();
        let fan1 = Fan::new(50);
        let fan2 = Fan::new(75);

        controller.add_fan(fan1);
        controller.add_fan(fan2);

        assert_eq!(controller.get_all_fan_statuses().len(), 2);

        controller.set_fan_speed(0, 60);
        assert_eq!(controller.get_fan_status(0), Some(&FanStatus::Off));

        controller.remove_fan(1);
        assert_eq!(controller.get_all_fan_statuses().len(), 1);
    }
}
