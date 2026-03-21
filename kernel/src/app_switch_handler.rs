extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AppSwitchHandler {
    current_app: Option<String>,
    app_history: Vec<String>,
}

impl AppSwitchHandler {
    pub fn new() -> Self {
        AppSwitchHandler {
            current_app: None,
            app_history: Vec::new(),
        }
    }

    pub fn switch_to(&mut self, app_name: &str) {
        if let Some(current) = &self.current_app {
            self.app_history.push(current.clone());
        }
        self.current_app = Some(app_name.to_string());
    }

    pub fn get_current_app(&self) -> Option<&String> {
        self.current_app.as_ref()
    }

    pub fn get_app_history(&self) -> &[String] {
        &self.app_history
    }

    pub fn clear_history(&mut self) {
        self.app_history.clear();
    }

    pub fn revert_to_previous(&mut self) -> Option<String> {
        if let Some(last_app) = self.app_history.pop() {
            self.current_app = Some(last_app.clone());
            Some(last_app)
        } else {
            None
        }
    }
}
