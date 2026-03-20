extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("Agent Task Planner module loaded!");
    0
}

pub struct AgentTaskPlanner {
    tasks: Vec<String>,
    completed_tasks: Vec<String>,
}

impl AgentTaskPlanner {
    pub fn new() -> Self {
        AgentTaskPlanner {
            tasks: Vec::new(),
            completed_tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, task_index: usize) -> Option<String> {
        if let Some(task) = self.tasks.get(task_index).cloned() {
            self.tasks.remove(task_index);
            self.completed_tasks.push(task.clone());
            Some(task)
        } else {
            None
        }
    }

    pub fn get_pending_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }

    pub fn get_completed_tasks(&self) -> Vec<String> {
        self.completed_tasks.clone()
    }

    pub fn clear_all_tasks(&mut self) {
        self.tasks.clear();
        self.completed_tasks.clear();
    }
}
