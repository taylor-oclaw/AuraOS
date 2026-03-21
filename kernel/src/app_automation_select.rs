extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn app_automation_select_init() {
    // Initialization logic for the module
}

pub extern "C" fn app_automation_select_exit() {
    // Cleanup logic for the module
}

pub struct AppAutomationSelect {
    tasks: Vec<String>,
    selected_task_index: usize,
}

impl AppAutomationSelect {
    pub fn new(tasks: Vec<String>) -> Self {
        AppAutomationSelect {
            tasks,
            selected_task_index: 0,
        }
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

    pub fn select_next_task(&mut self) {
        if !self.tasks.is_empty() {
            self.selected_task_index = (self.selected_task_index + 1) % self.tasks.len();
        }
    }

    pub fn select_previous_task(&mut self) {
        if !self.tasks.is_empty() {
            self.selected_task_index = if self.selected_task_index == 0 {
                self.tasks.len() - 1
            } else {
                self.selected_task_index - 1
            };
        }
    }

    pub fn get_selected_task(&self) -> Option<&String> {
        if !self.tasks.is_empty() {
            Some(&self.tasks[self.selected_task_index])
        } else {
            None
        }
    }
}
