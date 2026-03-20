extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Cleanup code for the module
}

pub struct AuraAgentPlayground {
    name: String,
    tasks: Vec<String>,
    status: String,
}

impl AuraAgentPlayground {
    pub fn new(name: &str) -> Self {
        AuraAgentPlayground {
            name: String::from(name),
            tasks: Vec::new(),
            status: String::from("Idle"),
        }
    }

    pub fn add_task(&mut self, task: &str) {
        self.tasks.push(String::from(task));
    }

    pub fn remove_task(&mut self, task_index: usize) -> Option<String> {
        if task_index < self.tasks.len() {
            Some(self.tasks.remove(task_index))
        } else {
            None
        }
    }

    pub fn get_tasks(&self) -> &Vec<String> {
        &self.tasks
    }

    pub fn set_status(&mut self, status: &str) {
        self.status = String::from(status);
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_agent_playground() {
        let mut agent = AuraAgentPlayground::new("Test Agent");

        assert_eq!(agent.get_name(), "Test Agent");
        assert_eq!(agent.get_status(), "Idle");
        assert!(agent.get_tasks().is_empty());

        agent.add_task("Task 1");
        agent.add_task("Task 2");

        assert_eq!(agent.get_tasks().len(), 2);
        assert_eq!(agent.get_tasks()[0], "Task 1");
        assert_eq!(agent.get_tasks()[1], "Task 2");

        let removed_task = agent.remove_task(0);
        assert_eq!(removed_task, Some(String::from("Task 1")));
        assert_eq!(agent.get_tasks().len(), 1);

        agent.set_status("Active");
        assert_eq!(agent.get_status(), "Active");
    }
}
