extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut task_manager = TaskManager::new();
    task_manager.create_task(String::from("Task1"), 10);
    task_manager.create_task(String::from("Task2"), 5);
    task_manager.list_tasks();
    task_manager.update_task_priority("Task1", 15);
    task_manager.delete_task("Task2");
    task_manager.list_tasks();

    loop {}
}

pub struct Task {
    name: String,
    priority: u32,
}

impl Task {
    pub fn new(name: String, priority: u32) -> Self {
        Task { name, priority }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_priority(&self) -> u32 {
        self.priority
    }

    pub fn set_priority(&mut self, priority: u32) {
        self.priority = priority;
    }
}

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

    pub fn create_task(&mut self, name: String, priority: u32) {
        let task = Task::new(name, priority);
        self.tasks.push(task);
    }

    pub fn list_tasks(&self) {
        for task in &self.tasks {
            println!("Task: {}, Priority: {}", task.get_name(), task.get_priority());
        }
    }

    pub fn update_task_priority(&mut self, name: &str, new_priority: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.get_name() == name) {
            task.set_priority(new_priority);
            true
        } else {
            false
        }
    }

    pub fn delete_task(&mut self, name: &str) -> bool {
        self.tasks.retain(|t| t.get_name() != name)
    }
}
