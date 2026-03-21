extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn app_automation_type_init() {
    // Initialization logic for the module
}

pub extern "C" fn app_automation_type_exit() {
    // Cleanup logic for the module
}

pub struct AppAutomationType {
    name: String,
    tasks: Vec<String>,
    enabled: bool,
}

impl AppAutomationType {
    pub fn new(name: &str) -> Self {
        AppAutomationType {
            name: String::from(name),
            tasks: Vec::new(),
            enabled: false,
        }
    }

    pub fn add_task(&mut self, task: &str) {
        self.tasks.push(String::from(task));
    }

    pub fn remove_task(&mut self, task: &str) -> bool {
        if let Some(index) = self.tasks.iter().position(|t| t == task) {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_automation_type() {
        let mut app = AppAutomationType::new("TestApp");
        assert_eq!(app.name, "TestApp");
        assert!(!app.is_enabled());

        app.add_task("Task1");
        app.add_task("Task2");
        assert_eq!(app.list_tasks(), vec![String::from("Task1"), String::from("Task2")]);

        assert!(app.remove_task("Task1"));
        assert_eq!(app.list_tasks(), vec![String::from("Task2")]);

        app.enable();
        assert!(app.is_enabled());

        app.disable();
        assert!(!app.is_enabled());
    }
}
