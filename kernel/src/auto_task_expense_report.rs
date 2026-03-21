extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let report = AutoTaskExpenseReport::new();
    report.add_task("Development", 1000);
    report.add_task("Testing", 500);
    report.add_task("Documentation", 300);
    report.add_task("Deployment", 200);
    report.add_task("Maintenance", 400);

    let total = report.total_expense();
    println!("Total Expense: {}", total); // This line is for demonstration purposes only

    loop {}
}

pub struct AutoTaskExpenseReport {
    tasks: Vec<(String, u32)>,
}

impl AutoTaskExpenseReport {
    pub fn new() -> Self {
        AutoTaskExpenseReport { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task_name: &str, expense: u32) {
        let name = String::from(task_name);
        self.tasks.push((name, expense));
    }

    pub fn get_task_expense(&self, task_name: &str) -> Option<u32> {
        for (task, expense) in &self.tasks {
            if task == task_name {
                return Some(*expense);
            }
        }
        None
    }

    pub fn total_expense(&self) -> u32 {
        self.tasks.iter().map(|(_, expense)| *expense).sum()
    }

    pub fn remove_task(&mut self, task_name: &str) {
        self.tasks.retain(|(task, _)| task != task_name);
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.iter().map(|(task, _)| task.clone()).collect()
    }
}
