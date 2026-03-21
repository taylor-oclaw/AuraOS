extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AppDomMutator {
    apps: Vec<String>,
}

impl AppDomMutator {
    pub fn new() -> Self {
        AppDomMutator { apps: Vec::new() }
    }

    pub fn add_app(&mut self, app_name: &str) {
        self.apps.push(String::from(app_name));
    }

    pub fn remove_app(&mut self, app_name: &str) {
        if let Some(index) = self.apps.iter().position(|x| x == app_name) {
            self.apps.remove(index);
        }
    }

    pub fn list_apps(&self) -> Vec<String> {
        self.apps.clone()
    }

    pub fn has_app(&self, app_name: &str) -> bool {
        self.apps.contains(&String::from(app_name))
    }

    pub fn count_apps(&self) -> usize {
        self.apps.len()
    }
}
