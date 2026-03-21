extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let reminder = MiniAppReminderDone::new(String::from("Task Reminder"));
    reminder.add_task(String::from("Buy groceries"), 1);
    reminder.add_task(String::from("Read a book"), 2);
    reminder.mark_task_done(1);
    reminder.remove_task(2);
    reminder.list_tasks();
}

pub struct MiniAppReminderDone {
    name: String,
    tasks: Vec<Task>,
}

impl MiniAppReminderDone {
    pub fn new(name: String) -> Self {
        MiniAppReminderDone {
            name,
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, description: String, id: usize) {
        let task = Task { id, description, done: false };
        self.tasks.push(task);
    }

    pub fn mark_task_done(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.done = true;
        }
    }

    pub fn remove_task(&mut self, id: usize) {
        self.tasks.retain(|t| t.id != id);
    }

    pub fn list_tasks(&self) {
        for task in &self.tasks {
            if task.done {
                println!("Task {} [Done]: {}", task.id, task.description);
            } else {
                println!("Task {}: {}", task.id, task.description);
            }
        }
    }

    pub fn get_task_count(&self) -> usize {
        self.tasks.len()
    }
}

struct Task {
    id: usize,
    description: String,
    done: bool,
}
