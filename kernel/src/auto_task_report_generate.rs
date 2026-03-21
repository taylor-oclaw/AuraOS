extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let report_generator = AutoTaskReportGenerator::new();
    report_generator.generate_report();
}

struct Task {
    name: String,
    status: String,
    priority: u8,
    duration: u32,
}

impl Task {
    fn new(name: &str, status: &str, priority: u8, duration: u32) -> Self {
        Task {
            name: String::from(name),
            status: String::from(status),
            priority,
            duration,
        }
    }

    fn is_completed(&self) -> bool {
        self.status == "completed"
    }

    fn is_high_priority(&self) -> bool {
        self.priority > 5
    }
}

pub struct AutoTaskReportGenerator {
    tasks: Vec<Task>,
}

impl AutoTaskReportGenerator {
    pub fn new() -> Self {
        AutoTaskReportGenerator {
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, name: &str) {
        self.tasks.retain(|t| t.name != name);
    }

    pub fn get_completed_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.is_completed()).collect()
    }

    pub fn get_high_priority_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.is_high_priority()).collect()
    }

    pub fn generate_report(&self) {
        for task in &self.tasks {
            // Simulate report generation logic
                     task.name, task.status, task.priority, task.duration;
        }
    }
}
