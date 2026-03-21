extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AppDomObserver {
    observed_apps: Vec<String>,
}

impl AppDomObserver {
    pub fn new() -> Self {
        AppDomObserver {
            observed_apps: Vec::new(),
        }
    }

    pub fn add_app(&mut self, app_name: &str) {
        if !self.observed_apps.contains(&app_name.to_string()) {
            self.observed_apps.push(app_name.to_string());
        }
    }

    pub fn remove_app(&mut self, app_name: &str) {
        self.observed_apps.retain(|name| name != app_name);
    }

    pub fn is_observing(&self, app_name: &str) -> bool {
        self.observed_apps.contains(&app_name.to_string())
    }

    pub fn list_observed_apps(&self) -> Vec<String> {
        self.observed_apps.clone()
    }

    pub fn clear_all(&mut self) {
        self.observed_apps.clear();
    }
}
