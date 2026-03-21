extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct AutoTaskStatusUpdate {
    task_id: u32,
    status: String,
    logs: Vec<String>,
}

impl AutoTaskStatusUpdate {
    pub fn new(task_id: u32) -> Self {
        AutoTaskStatusUpdate {
            task_id,
            status: String::from("Pending"),
            logs: Vec::new(),
        }
    }

    pub fn update_status(&mut self, new_status: &str) {
        self.status = String::from(new_status);
    }

    pub fn add_log(&mut self, log_message: &str) {
        self.logs.push(String::from(log_message));
    }

    pub fn get_task_id(&self) -> u32 {
        self.task_id
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn get_logs(&self) -> &[String] {
        &self.logs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_task_status_update() {
        let mut task = AutoTaskStatusUpdate::new(1);
        assert_eq!(task.get_task_id(), 1);
        assert_eq!(task.get_status(), "Pending");
        assert!(task.get_logs().is_empty());

        task.update_status("Running");
        assert_eq!(task.get_status(), "Running");

        task.add_log("Started processing");
        task.add_log("Completed step 1");
        assert_eq!(task.get_logs().len(), 2);
        assert_eq!(task.get_logs()[0], "Started processing");
        assert_eq!(task.get_logs()[1], "Completed step 1");
    }
}
