extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut prep = MeetingPrepAuto::new();
    prep.add_task(String::from("Review Agenda"));
    prep.add_task(String::from("Send Invitations"));
    prep.add_task(String::from("Prepare Slides"));
    prep.add_task(String::from("Check Equipment"));
    prep.add_task(String::from("Confirm Participants"));

    let tasks = prep.get_tasks();
    for task in tasks.iter() {
        // Simulate processing each task
    }

    loop {}
}

pub struct MeetingPrepAuto {
    tasks: Vec<String>,
}

impl MeetingPrepAuto {
    pub fn new() -> Self {
        MeetingPrepAuto { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, index: usize) -> Option<String> {
        if index < self.tasks.len() {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    pub fn get_tasks(&self) -> &Vec<String> {
        &self.tasks
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    pub fn has_task(&self, task: &str) -> bool {
        self.tasks.iter().any(|t| t == task)
    }
}
