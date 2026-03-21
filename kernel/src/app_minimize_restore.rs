extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn app_minimize_restore_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn app_minimize_restore_exit() {
    // Cleanup logic for the module
}

pub struct AppMinimizeRestore {
    apps: Vec<String>,
    minimized_apps: Vec<String>,
}

impl AppMinimizeRestore {
    pub fn new() -> Self {
        AppMinimizeRestore {
            apps: Vec::new(),
            minimized_apps: Vec::new(),
        }
    }

    pub fn add_app(&mut self, app_name: &str) {
        self.apps.push(app_name.to_string());
    }

    pub fn minimize_app(&mut self, app_name: &str) -> bool {
        if let Some(index) = self.apps.iter().position(|x| x == app_name) {
            let app = self.apps.remove(index);
            self.minimized_apps.push(app);
            true
        } else {
            false
        }
    }

    pub fn restore_app(&mut self, app_name: &str) -> bool {
        if let Some(index) = self.minimized_apps.iter().position(|x| x == app_name) {
            let app = self.minimized_apps.remove(index);
            self.apps.push(app);
            true
        } else {
            false
        }
    }

    pub fn list_apps(&self) -> Vec<String> {
        self.apps.clone()
    }

    pub fn list_minimized_apps(&self) -> Vec<String> {
        self.minimized_apps.clone()
    }
}
