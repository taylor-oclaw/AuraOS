extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn app_focus_manager_init() {
    // Initialization logic for the app focus manager
}

pub extern "C" fn app_focus_manager_exit() {
    // Cleanup logic for the app focus manager
}

pub struct AppFocusManager {
    focused_app: Option<String>,
    apps: Vec<String>,
}

impl AppFocusManager {
    pub fn new() -> Self {
        AppFocusManager {
            focused_app: None,
            apps: Vec::new(),
        }
    }

    pub fn add_app(&mut self, app_name: &str) {
        if !self.apps.contains(&app_name.to_string()) {
            self.apps.push(app_name.to_string());
        }
    }

    pub fn remove_app(&mut self, app_name: &str) {
        self.apps.retain(|app| app != app_name);
        if let Some(focused) = &self.focused_app {
            if focused == app_name {
                self.focused_app = None;
            }
        }
    }

    pub fn focus_app(&mut self, app_name: &str) -> bool {
        if self.apps.contains(&app_name.to_string()) {
            self.focused_app = Some(app_name.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_focused_app(&self) -> Option<&String> {
        self.focused_app.as_ref()
    }

    pub fn list_apps(&self) -> &Vec<String> {
        &self.apps
    }
}
