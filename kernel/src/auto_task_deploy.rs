extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod task {
    use super::*;

    pub struct Task {
        id: u32,
        name: String,
        status: String,
        dependencies: Vec<u32>,
        priority: u8,
    }

    impl Task {
        pub fn new(id: u32, name: &str, status: &str, dependencies: Vec<u32>, priority: u8) -> Self {
            Task {
                id,
                name: String::from(name),
                status: String::from(status),
                dependencies,
                priority,
            }
        }

        pub fn get_id(&self) -> u32 {
            self.id
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_status(&self) -> &str {
            &self.status
        }

        pub fn set_status(&mut self, status: &str) {
            self.status = String::from(status);
        }

        pub fn add_dependency(&mut self, dependency_id: u32) {
            if !self.dependencies.contains(&dependency_id) {
                self.dependencies.push(dependency_id);
            }
        }

        pub fn remove_dependency(&mut self, dependency_id: u32) {
            self.dependencies.retain(|&id| id != dependency_id);
        }

        pub fn get_dependencies(&self) -> &Vec<u32> {
            &self.dependencies
        }

        pub fn get_priority(&self) -> u8 {
            self.priority
        }

        pub fn set_priority(&mut self, priority: u8) {
            self.priority = priority;
        }
    }
}

pub struct AutoTaskDeploy {
    tasks: Vec<task::Task>,
}

impl AutoTaskDeploy {
    pub fn new() -> Self {
        AutoTaskDeploy {
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: task::Task) {
        self.tasks.push(task);
    }

    pub fn get_task_by_id(&self, id: u32) -> Option<&task::Task> {
        self.tasks.iter().find(|t| t.get_id() == id)
    }

    pub fn update_task_status(&mut self, id: u32, status: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.get_id() == id) {
            task.set_status(status);
        }
    }

    pub fn remove_task(&mut self, id: u32) -> Option<task::Task> {
        self.tasks.retain(|t| t.get_id() != id);
        self.tasks.into_iter().find(|t| t.get_id() == id)
    }

    pub fn list_tasks(&self) -> &Vec<task::Task> {
        &self.tasks
    }
}
