extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod task {
    pub struct Task {
        id: u32,
        name: String,
        priority: u8,
        status: TaskStatus,
    }

    impl Task {
        pub fn new(id: u32, name: &str, priority: u8) -> Self {
            Task {
                id,
                name: String::from(name),
                priority,
                status: TaskStatus::Pending,
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

        pub fn get_status(&self) -> TaskStatus {
            self.status
        }

        pub fn set_status(&mut self, status: TaskStatus) {
            self.status = status;
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum TaskStatus {
        Pending,
        Running,
        Completed,
        Blocked,
        Suspended,
    }
}

pub struct MeshTaskScheduler {
    tasks: Vec<task::Task>,
}

impl MeshTaskScheduler {
    pub fn new() -> Self {
        MeshTaskScheduler {
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: task::Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, id: u32) -> Option<task::Task> {
        let pos = self.tasks.iter().position(|t| t.get_id() == id);
        match pos {
            Some(index) => Some(self.tasks.remove(index)),
            None => None,
        }
    }

    pub fn get_task_by_id(&self, id: u32) -> Option<&task::Task> {
        self.tasks.iter().find(|t| t.get_id() == id)
    }

    pub fn list_tasks(&self) -> &[task::Task] {
        &self.tasks
    }

    pub fn update_task_status(&mut self, id: u32, status: task::TaskStatus) -> bool {
        if let Some(task) = self.get_task_by_id(id) {
            let index = self.tasks.iter().position(|t| t.get_id() == id).unwrap();
            self.tasks[index].set_status(status);
            true
        } else {
            false
        }
    }
}
