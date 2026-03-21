extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let mut report = AutoTaskBugReport::new(String::from("Initial Report"));
    report.add_task("Task 1");
    report.add_task("Task 2");
    report.mark_task_completed(0);
    report.remove_task(1);
    report.print_report();
}

pub struct AutoTaskBugReport {
    title: String,
    tasks: Vec<Task>,
}

impl AutoTaskBugReport {
    pub fn new(title: String) -> Self {
        AutoTaskBugReport {
            title,
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, description: &str) {
        let task = Task {
            description: description.to_string(),
            completed: false,
        };
        self.tasks.push(task);
    }

    pub fn mark_task_completed(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = true;
        }
    }

    pub fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        }
    }

    pub fn print_report(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "Completed" } else { "Pending" };
        }
    }

    pub fn get_task_count(&self) -> usize {
        self.tasks.len()
    }
}

struct Task {
    description: String,
    completed: bool,
}
