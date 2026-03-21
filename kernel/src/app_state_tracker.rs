extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AppStateTracker {
    app_states: Vec<(String, String)>,
}

impl AppStateTracker {
    pub fn new() -> Self {
        AppStateTracker {
            app_states: Vec::new(),
        }
    }

    pub fn add_app_state(&mut self, app_name: &str, state: &str) {
        let app_name = String::from(app_name);
        let state = String::from(state);
        self.app_states.push((app_name, state));
    }

    pub fn get_app_state(&self, app_name: &str) -> Option<&String> {
        for (name, state) in &self.app_states {
            if name == app_name {
                return Some(state);
            }
        }
        None
    }

    pub fn remove_app_state(&mut self, app_name: &str) {
        self.app_states.retain(|(name, _)| name != app_name);
    }

    pub fn list_all_apps(&self) -> Vec<&String> {
        self.app_states.iter().map(|(name, _)| name).collect()
    }

    pub fn update_app_state(&mut self, app_name: &str, new_state: &str) {
        if let Some(index) = self.app_states.iter().position(|(name, _)| name == app_name) {
            let (_, state) = &mut self.app_states[index];
            *state = String::from(new_state);
        }
    }
}
