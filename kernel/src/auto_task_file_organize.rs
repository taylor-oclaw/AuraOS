extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut organizer = AutoTaskFileOrganizer::new();
    organizer.add_task("task1", "/path/to/task1");
    organizer.add_task("task2", "/path/to/task2");
    organizer.list_tasks();
    organizer.remove_task("task1");
    organizer.list_tasks();
}

pub struct AutoTaskFileOrganizer {
    tasks: Vec<(String, String)>,
}

impl AutoTaskFileOrganizer {
    pub fn new() -> Self {
        AutoTaskFileOrganizer { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: &str, path: &str) {
        let task_name = String::from(name);
        let task_path = String::from(path);
        self.tasks.push((task_name, task_path));
    }

    pub fn remove_task(&mut self, name: &str) {
        self.tasks.retain(|(task_name, _)| task_name != name);
    }

    pub fn list_tasks(&self) {
        for (name, path) in &self.tasks {
            // Simulate printing task information
            println!("Task Name: {}, Path: {}", name, path); // This is a placeholder for actual kernel logging
        }
    }

    pub fn get_task_path(&self, name: &str) -> Option<&String> {
        self.tasks.iter().find(|(task_name, _)| task_name == name).map(|(_, path)| path)
    }

    pub fn has_task(&self, name: &str) -> bool {
        self.tasks.iter().any(|(task_name, _)| task_name == name)
    }
}
