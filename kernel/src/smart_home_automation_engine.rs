extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    let engine = SmartHomeAutomationEngine::new();
    engine.initialize_system();
    loop {
        engine.check_sensors();
        engine.execute_tasks();
        engine.log_status();
    }
}

pub struct SmartHomeAutomationEngine {
    devices: Vec<String>,
    tasks: Vec<String>,
    status_log: String,
}

impl SmartHomeAutomationEngine {
    pub fn new() -> Self {
        SmartHomeAutomationEngine {
            devices: Vec::new(),
            tasks: Vec::new(),
            status_log: String::from("System initialized."),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices.push(String::from(device_name));
        self.status_log.push_str(&format!("Device {} added.\n", device_name));
    }

    pub fn remove_device(&mut self, device_name: &str) {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
            self.status_log.push_str(&format!("Device {} removed.\n", device_name));
        }
    }

    pub fn add_task(&mut self, task_description: &str) {
        self.tasks.push(String::from(task_description));
        self.status_log.push_str(&format!("Task added: {}\n", task_description));
    }

    pub fn remove_task(&mut self, task_description: &str) {
        if let Some(index) = self.tasks.iter().position(|t| t == task_description) {
            self.tasks.remove(index);
            self.status_log.push_str(&format!("Task removed: {}\n", task_description));
        }
    }

    pub fn log_status(&self) {
        // Simulate logging to a console or file
        println!("{}", self.status_log); // Assuming println! is available for demonstration purposes
    }

    pub fn initialize_system(&mut self) {
        self.add_device("Light");
        self.add_device("Thermostat");
        self.add_task("Turn on lights at 7 AM");
        self.add_task("Set thermostat to 22°C");
        self.status_log.push_str("System initialized with default devices and tasks.\n");
    }

    pub fn check_sensors(&self) {
        // Simulate sensor checks
        self.status_log.push_str("Sensor data checked.\n");
    }

    pub fn execute_tasks(&mut self) {
        for task in &self.tasks {
            self.status_log.push_str(&format!("Executing task: {}\n", task));
        }
    }
}
