extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneThreadContext {
    thread_id: u32,
    priority: u8,
    state: String,
    stack_size: usize,
    tasks: Vec<String>,
}

impl ToneThreadContext {
    pub fn new(thread_id: u32, priority: u8, stack_size: usize) -> Self {
        ToneThreadContext {
            thread_id,
            priority,
            state: String::from("Ready"),
            stack_size,
            tasks: Vec::new(),
        }
    }

    pub fn get_thread_id(&self) -> u32 {
        self.thread_id
    }

    pub fn set_priority(&mut self, new_priority: u8) {
        self.priority = new_priority;
    }

    pub fn add_task(&mut self, task_name: &str) {
        self.tasks.push(String::from(task_name));
    }

    pub fn remove_task(&mut self, task_name: &str) {
        self.tasks.retain(|task| task != task_name);
    }

    pub fn get_tasks(&self) -> &[String] {
        &self.tasks
    }
}
