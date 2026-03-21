extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut task = AutoTaskResearch::new();
    task.initialize_tasks();
    task.start_task(0);
    task.stop_task(1);
    task.list_all_tasks();
    task.get_task_status(2);

    loop {}
}

pub struct AutoTaskResearch {
    tasks: Vec<Task>,
}

impl AutoTaskResearch {
    pub fn new() -> Self {
        AutoTaskResearch {
            tasks: Vec::new(),
        }
    }

    pub fn initialize_tasks(&mut self) {
        for i in 0..5 {
            let task_name = String::from("info");
            self.tasks.push(Task::new(task_name, TaskStatus::Initialized));
        }
    }

    pub fn start_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.status = TaskStatus::Running;
        } else {
        }
    }

    pub fn stop_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.status = TaskStatus::Stopped;
        } else {
        }
    }

    pub fn list_all_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
        }
    }

    pub fn get_task_status(&self, index: usize) -> Option<TaskStatus> {
        self.tasks.get(index).map(|task| task.status)
    }
}

#[derive(Debug)]
pub enum TaskStatus {
    Initialized,
    Running,
    Stopped,
}

pub struct Task {
    name: String,
    status: TaskStatus,
}

impl Task {
    pub fn new(name: String, status: TaskStatus) -> Self {
        Task { name, status }
    }
}
