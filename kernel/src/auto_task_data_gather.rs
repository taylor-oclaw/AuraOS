extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct AutoTaskDataGather {
    tasks: Vec<String>,
    data: Vec<u8>,
    status: String,
}

impl AutoTaskDataGather {
    pub fn new() -> Self {
        AutoTaskDataGather {
            tasks: Vec::new(),
            data: Vec::new(),
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

    pub fn gather_data(&mut self, new_data: &[u8]) {
        self.data.extend_from_slice(new_data);
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_task_data_gather() {
        let mut gather = AutoTaskDataGather::new();

        // Test adding tasks
        gather.add_task("Task1");
        gather.add_task("Task2");
        assert_eq!(gather.tasks.len(), 2);

        // Test removing a task
        let removed_task = gather.remove_task(0);
        assert_eq!(removed_task, Some(String::from("Task1")));
        assert_eq!(gather.tasks.len(), 1);

        // Test gathering data
        let data = [1, 2, 3];
        gather.gather_data(&data);
        assert_eq!(gather.data, vec![1, 2, 3]);

        // Test clearing data
        gather.clear_data();
        assert_eq!(gather.data.len(), 0);

        // Test getting status
        assert_eq!(gather.get_status(), "Idle");
    }
}
