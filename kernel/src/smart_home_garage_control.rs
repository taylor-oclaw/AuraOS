extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let garage = GarageControl::new();
    garage.open_door();
    garage.close_door();
    garage.lock_garage();
    garage.unlock_garage();
    garage.status();
}

pub struct GarageControl {
    door_open: bool,
    locked: bool,
    status_log: Vec<String>,
}

impl GarageControl {
    pub fn new() -> Self {
        GarageControl {
            door_open: false,
            locked: true,
            status_log: Vec::new(),
        }
    }

    pub fn open_door(&mut self) {
        if !self.locked {
            self.door_open = true;
            self.log_status("Door opened");
        } else {
            self.log_status("Cannot open door, garage is locked");
        }
    }

    pub fn close_door(&mut self) {
        self.door_open = false;
        self.log_status("Door closed");
    }

    pub fn lock_garage(&mut self) {
        if !self.door_open {
            self.locked = true;
            self.log_status("Garage locked");
        } else {
            self.log_status("Cannot lock garage, door is open");
        }
    }

    pub fn unlock_garage(&mut self) {
        self.locked = false;
        self.log_status("Garage unlocked");
    }

    pub fn status(&self) -> String {
        let mut status = String::from("Garage Status:\n");
        status.push_str(&String::from("info"));
        status.push_str(&String::from("info"));
        status.push_str("Status Log:\n");
        for log in &self.status_log {
            status.push_str(log);
            status.push('\n');
        }
        status
    }

    fn log_status(&mut self, message: &str) {
        let entry = String::from(message);
        self.status_log.push(entry);
    }
}
