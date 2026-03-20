extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn agent_bulkhead_init() {
    // Initialization logic for the module
}

pub extern "C" fn agent_bulkhead_exit() {
    // Cleanup logic for the module
}

pub struct AgentBulkhead {
    tasks: Vec<String>,
    max_tasks: usize,
}

impl AgentBulkhead {
    pub fn new(max_tasks: usize) -> Self {
        AgentBulkhead {
            tasks: Vec::new(),
            max_tasks,
        }
    }

    pub fn add_task(&mut self, task: String) -> Result<(), &'static str> {
        if self.tasks.len() >= self.max_tasks {
            Err("Maximum number of tasks reached")
        } else {
            self.tasks.push(task);
            Ok(())
        }
    }

    pub fn remove_task(&mut self, index: usize) -> Option<String> {
        if index < self.tasks.len() {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    pub fn get_tasks(&self) -> &Vec<String> {
        &self.tasks
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_bulkhead() {
        let mut bulkhead = AgentBulkhead::new(3);
        assert_eq!(bulkhead.task_count(), 0);

        assert!(bulkhead.add_task(String::from("task1")).is_ok());
        assert!(bulkhead.add_task(String::from("task2")).is_ok());
        assert!(bulkhead.add_task(String::from("task3")).is_ok());

        assert_eq!(bulkhead.task_count(), 3);
        assert_eq!(bulkhead.get_tasks().len(), 3);

        assert!(bulkhead.add_task(String::from("task4")).is_err());

        assert_eq!(bulkhead.remove_task(1), Some(String::from("task2")));
        assert_eq!(bulkhead.task_count(), 2);

        bulkhead.clear_tasks();
        assert_eq!(bulkhead.task_count(), 0);
    }
}
