extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn app_automation_drag_init() {
    // Initialization logic for the module
}

pub extern "C" fn app_automation_drag_exit() {
    // Cleanup logic for the module
}

pub struct AppAutomationDrag {
    actions: Vec<String>,
    current_step: usize,
}

impl AppAutomationDrag {
    pub fn new() -> Self {
        AppAutomationDrag {
            actions: Vec::new(),
            current_step: 0,
        }
    }

    pub fn add_action(&mut self, action: String) {
        self.actions.push(action);
    }

    pub fn get_current_action(&self) -> Option<&String> {
        if self.current_step < self.actions.len() {
            Some(&self.actions[self.current_step])
        } else {
            None
        }
    }

    pub fn next_action(&mut self) -> Option<&String> {
        if self.current_step < self.actions.len() {
            let action = &self.actions[self.current_step];
            self.current_step += 1;
            Some(action)
        } else {
            None
        }
    }

    pub fn reset_actions(&mut self) {
        self.current_step = 0;
    }

    pub fn has_more_actions(&self) -> bool {
        self.current_step < self.actions.len()
    }
}
