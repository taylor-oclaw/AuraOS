extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AppLauncher {
    apps: Vec<String>,
}

impl AppLauncher {
    pub fn new() -> Self {
        AppLauncher { apps: Vec::new() }
    }

    pub fn add_app(&mut self, app_name: &str) {
        self.apps.push(app_name.to_string());
    }

    pub fn remove_app(&mut self, app_name: &str) {
        if let Some(index) = self.apps.iter().position(|x| x == app_name) {
            self.apps.remove(index);
        }
    }

    pub fn list_apps(&self) -> Vec<String> {
        self.apps.clone()
    }

    pub fn launch_app(&self, app_name: &str) -> bool {
        self.apps.contains(&app_name.to_string())
    }

    pub fn count_apps(&self) -> usize {
        self.apps.len()
    }
}
