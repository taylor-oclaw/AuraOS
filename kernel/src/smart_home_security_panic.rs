extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut security_system = SmartHomeSecurityPanic::new();
    security_system.initialize();
    loop {
        security_system.monitor_sensors();
        if security_system.is_alert() {
            security_system.trigger_alarm();
        }
        security_system.log_status();
    }
}

pub struct SmartHomeSecurityPanic {
    sensors: Vec<String>,
    alerts: bool,
    status_log: Vec<String>,
}

impl SmartHomeSecurityPanic {
    pub fn new() -> Self {
        SmartHomeSecurityPanic {
            sensors: Vec::new(),
            alerts: false,
            status_log: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.sensors.push(String::from("Motion Sensor"));
        self.sensors.push(String::from("Door Sensor"));
        self.sensors.push(String::from("Window Sensor"));
        self.status_log.push(String::from("System Initialized"));
    }

    pub fn monitor_sensors(&mut self) {
        for sensor in &self.sensors {
            // Simulate sensor monitoring
            if sensor == "Motion Sensor" {
                self.alerts = true;
                self.status_log.push(format!("Alert: Motion detected by {}", sensor));
            }
        }
    }

    pub fn is_alert(&self) -> bool {
        self.alerts
    }

    pub fn trigger_alarm(&mut self) {
        if self.is_alert() {
            self.status_log.push(String::from("Alarm Triggered"));
            // Simulate alarm action
            self.alerts = false;
        }
    }

    pub fn log_status(&self) {
        for entry in &self.status_log {
            // Log status to a file or console (simulated here)
            println!("{}", entry);
        }
    }
}
