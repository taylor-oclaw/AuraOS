extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    let doorbell = SmartHomeDoorbell::new();
    doorbell.ring();
    doorbell.set_name(String::from("Front Door"));
    doorbell.add_notification(String::from("Alert: Someone at the door!"));
    doorbell.send_notifications();
    loop {}
}

pub struct SmartHomeDoorbell {
    name: String,
    notifications: Vec<String>,
    is_ringing: bool,
}

impl SmartHomeDoorbell {
    pub fn new() -> Self {
        SmartHomeDoorbell {
            name: String::from("Unnamed Doorbell"),
            notifications: Vec::new(),
            is_ringing: false,
        }
    }

    pub fn ring(&mut self) {
        self.is_ringing = true;
    }

    pub fn stop_ringing(&mut self) {
        self.is_ringing = false;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_notification(&mut self, notification: String) {
        self.notifications.push(notification);
    }

    pub fn send_notifications(&self) {
        for notification in &self.notifications {
        }
    }
}
