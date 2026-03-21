extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    let mut review = AutoTaskCodeReview::new();
    review.add_task("task1".into(), "code1".into());
    review.add_task("task2".into(), "code2".into());
    review.review_task("task1");
    review.list_tasks();
    review.remove_task("task2");
    loop {}
}

pub struct AutoTaskCodeReview {
    tasks: Vec<(String, String)>,
}

impl AutoTaskCodeReview {
    pub fn new() -> Self {
        AutoTaskCodeReview { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task_name: String, code: String) {
        self.tasks.push((task_name, code));
    }

    pub fn review_task(&self, task_name: &str) -> Option<&String> {
        self.tasks.iter().find(|&&(ref name, _)| name == task_name).map(|(_, code)| code)
    }

    pub fn list_tasks(&self) {
        for (task_name, _) in &self.tasks {
            // Simulate listing tasks
        }
    }

    pub fn remove_task(&mut self, task_name: &str) -> bool {
        let pos = self.tasks.iter().position(|&(ref name, _)| name == task_name);
        if let Some(index) = pos {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_task_count(&self) -> usize {
        self.tasks.len()
    }
}
