extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod priority_engine_core {
    extern crate alloc;
    use alloc::string::String;
    use alloc::vec::Vec;

    pub struct Task {
        id: u32,
        name: String,
        priority: u8,
        status: String,
    }

    impl Task {
        pub fn new(id: u32, name: &str, priority: u8) -> Self {
            Task {
                id,
                name: String::from(name),
                priority,
                status: String::from("Pending"),
            }
        }

        pub fn get_id(&self) -> u32 {
            self.id
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_priority(&self) -> u8 {
            self.priority
        }

        pub fn get_status(&self) -> &str {
            &self.status
        }

        pub fn set_status(&mut self, status: &str) {
            self.status = String::from(status);
        }
    }

    pub struct PriorityEngine {
        tasks: Vec<Task>,
    }

    impl PriorityEngine {
        pub fn new() -> Self {
            PriorityEngine {
                tasks: Vec::new(),
            }
        }

        pub fn add_task(&mut self, task: Task) {
            self.tasks.push(task);
        }

        pub fn remove_task(&mut self, id: u32) {
            self.tasks.retain(|t| t.id != id);
        }

        pub fn get_tasks_by_priority(&self, priority: u8) -> Vec<&Task> {
            self.tasks.iter().filter(|t| t.priority == priority).collect()
        }

        pub fn get_task_by_id(&self, id: u32) -> Option<&Task> {
            self.tasks.iter().find(|t| t.id == id)
        }
    }
}
