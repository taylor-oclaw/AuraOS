extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let scheduler = NpuScheduler::new();
    scheduler.initialize();
    scheduler.add_task("Task1".into());
    scheduler.add_task("Task2".into());
    scheduler.run_next_task();
    scheduler.remove_task("Task1");
    scheduler.shutdown();
}

pub struct NpuScheduler {
    tasks: Vec<String>,
    current_task_index: usize,
}

impl NpuScheduler {
    pub fn new() -> Self {
        NpuScheduler {
            tasks: Vec::new(),
            current_task_index: 0,
        }
    }

    pub fn initialize(&mut self) {
        // Initialize the scheduler
    }

    pub fn add_task(&mut self, task_name: String) {
        // Add a new task to the scheduler
        self.tasks.push(task_name);
    }

    pub fn remove_task(&mut self, task_name: &str) {
        // Remove a task from the scheduler
        if let Some(index) = self.tasks.iter().position(|t| t == task_name) {
            self.tasks.remove(index);
        } else {
        }
    }

    pub fn run_next_task(&mut self) {
        // Run the next task in the queue
        if !self.tasks.is_empty() {
            let current_task = &self.tasks[self.current_task_index];
            self.current_task_index = (self.current_task_index + 1) % self.tasks.len();
        } else {
        }
    }

    pub fn shutdown(&mut self) {
        // Shutdown the scheduler
        self.tasks.clear();
        self.current_task_index = 0;
    }
}
