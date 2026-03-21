extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("Hello from Rust kernel module!");
    0
}

pub struct PerfAppPreloader {
    apps: Vec<String>,
    loaded_apps: usize,
}

impl PerfAppPreloader {
    pub fn new() -> Self {
        PerfAppPreloader {
            apps: Vec::new(),
            loaded_apps: 0,
        }
    }

    pub fn add_app(&mut self, app_name: &str) {
        self.apps.push(String::from(app_name));
    }

    pub fn remove_app(&mut self, app_name: &str) -> bool {
        if let Some(index) = self.apps.iter().position(|x| x == app_name) {
            self.apps.remove(index);
            true
        } else {
            false
        }
    }

    pub fn load_apps(&mut self) {
        for _ in 0..self.apps.len() {
            // Simulate loading an app
            self.loaded_apps += 1;
        }
    }

    pub fn get_loaded_app_count(&self) -> usize {
        self.loaded_apps
    }

    pub fn list_apps(&self) -> Vec<String> {
        self.apps.clone()
    }
}
