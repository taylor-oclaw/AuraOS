extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn app_controller_init() {
    // Initialization logic for the app controller module
}

#[no_mangle]
pub extern "C" fn app_controller_exit() {
    // Cleanup logic for the app controller module
}

pub struct AppController {
    apps: Vec<String>,
    active_app: Option<usize>,
}

impl AppController {
    pub fn new() -> Self {
        AppController {
            apps: Vec::new(),
            active_app: None,
        }
    }

    pub fn add_app(&mut self, app_name: &str) {
        self.apps.push(String::from(app_name));
    }

    pub fn remove_app(&mut self, app_name: &str) -> bool {
        if let Some(index) = self.apps.iter().position(|app| *app == app_name) {
            self.apps.remove(index);
            if self.active_app == Some(index) {
                self.active_app = None;
            }
            true
        } else {
            false
        }
    }

    pub fn activate_app(&mut self, app_name: &str) -> bool {
        if let Some(index) = self.apps.iter().position(|app| *app == app_name) {
            self.active_app = Some(index);
            true
        } else {
            false
        }
    }

    pub fn deactivate_app(&mut self) {
        self.active_app = None;
    }

    pub fn get_active_app(&self) -> Option<&str> {
        self.active_app.map(|index| &self.apps[index])
    }
}
