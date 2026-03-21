extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn app_close_handler_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn app_close_handler_exit() {
    // Cleanup logic for the module
}

pub struct AppCloseHandler {
    apps: Vec<String>,
}

impl AppCloseHandler {
    pub fn new() -> Self {
        AppCloseHandler { apps: Vec::new() }
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

    pub fn is_app_registered(&self, app_name: &str) -> bool {
        self.apps.contains(&app_name.to_string())
    }

    pub fn clear_all_apps(&mut self) {
        self.apps.clear();
    }
}
