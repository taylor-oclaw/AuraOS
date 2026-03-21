extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let reminder = Reminder::new("Thank you for using our AI-native OS!".to_string());
    reminder.add_task("Check system updates".to_string());
    reminder.add_task("Backup important files".to_string());
    reminder.add_task("Review security settings".to_string());

    for task in reminder.get_tasks() {
    }

    reminder.complete_task(1);
    for (index, task) in reminder.get_completed_tasks().iter().enumerate() {
    }
}

pub struct Reminder {
    message: String,
    tasks: Vec<String>,
    completed_tasks: Vec<String>,
}

impl Reminder {
    pub fn new(message: String) -> Self {
        Reminder {
            message,
            tasks: Vec::new(),
            completed_tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_tasks(&self) -> &[String] {
        &self.tasks
    }

    pub fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get(index) {
            self.completed_tasks.push(task.clone());
            self.tasks.remove(index);
        }
    }

    pub fn get_completed_tasks(&self) -> &[String] {
        &self.completed_tasks
    }
}
