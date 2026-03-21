extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut task_doc = AutoTaskDocCreate::new();
    task_doc.add_task("Document creation");
    task_doc.add_task("Code review");
    task_doc.add_task("Testing");
    task_doc.add_task("Deployment");
    task_doc.add_task("Monitoring");

    if task_doc.is_task_complete("Document creation") {
    } else {
    }

    let all_tasks = task_doc.get_all_tasks();
    for task in all_tasks.iter() {
    }
}

pub struct AutoTaskDocCreate {
    tasks: Vec<String>,
    completed_tasks: Vec<String>,
}

impl AutoTaskDocCreate {
    pub fn new() -> Self {
        AutoTaskDocCreate {
            tasks: Vec::new(),
            completed_tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: &str) {
        self.tasks.push(String::from(task));
    }

    pub fn complete_task(&mut self, task: &str) -> bool {
        if let Some(index) = self.tasks.iter().position(|t| t == task) {
            let completed_task = self.tasks.remove(index);
            self.completed_tasks.push(completed_task);
            true
        } else {
            false
        }
    }

    pub fn is_task_complete(&self, task: &str) -> bool {
        self.completed_tasks.contains(&String::from(task))
    }

    pub fn get_all_tasks(&self) -> Vec<String> {
        let mut all_tasks = self.tasks.clone();
        all_tasks.extend_from_slice(&self.completed_tasks);
        all_tasks
    }

    pub fn get_pending_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }
}
