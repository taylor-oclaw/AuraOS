extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshTaskResume {
    tasks: Vec<String>,
}

impl MeshTaskResume {
    pub fn new() -> Self {
        MeshTaskResume { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task_name: &str) {
        self.tasks.push(String::from(task_name));
    }

    pub fn remove_task(&mut self, task_name: &str) {
        if let Some(index) = self.tasks.iter().position(|t| t == task_name) {
            self.tasks.remove(index);
        }
    }

    pub fn get_tasks(&self) -> &[String] {
        &self.tasks
    }

    pub fn has_task(&self, task_name: &str) -> bool {
        self.tasks.contains(&String::from(task_name))
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }
}
