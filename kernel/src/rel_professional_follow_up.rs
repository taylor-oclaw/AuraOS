extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_professional_follow_up_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rel_professional_follow_up_exit() {
    // Cleanup logic for the module
}

pub struct ProfessionalFollowUp {
    tasks: Vec<String>,
    completed_tasks: usize,
}

impl ProfessionalFollowUp {
    pub fn new() -> Self {
        ProfessionalFollowUp {
            tasks: Vec::new(),
            completed_tasks: 0,
        }
    }

    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self) {
        if !self.tasks.is_empty() {
            self.completed_tasks += 1;
            self.tasks.remove(0); // Assuming tasks are completed in the order they are added
        }
    }

    pub fn get_total_tasks(&self) -> usize {
        self.tasks.len()
    }

    pub fn get_completed_tasks(&self) -> usize {
        self.completed_tasks
    }

    pub fn list_pending_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }
}
