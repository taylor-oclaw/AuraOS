extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut scheduler = NotifyScheduleAdaptive::new();
    scheduler.add_task("task1".into(), 5);
    scheduler.add_task("task2".into(), 3);
    scheduler.add_task("task3".into(), 8);

    while !scheduler.is_empty() {
        if let Some(task) = scheduler.get_next_task() {
            println!("Running task: {}", task.name);
            scheduler.complete_task(&task.name);
        }
    }
}

pub struct NotifyScheduleAdaptive {
    tasks: Vec<Task>,
}

impl NotifyScheduleAdaptive {
    pub fn new() -> Self {
        NotifyScheduleAdaptive { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: String, priority: u32) {
        let task = Task { name, priority };
        self.tasks.push(task);
        self.tasks.sort_by_key(|t| t.priority);
    }

    pub fn get_next_task(&self) -> Option<&Task> {
        self.tasks.first()
    }

    pub fn complete_task(&mut self, name: &str) {
        if let Some(index) = self.tasks.iter().position(|t| t.name == name) {
            self.tasks.remove(index);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.iter().map(|t| t.name.clone()).collect()
    }
}

struct Task {
    name: String,
    priority: u32,
}
