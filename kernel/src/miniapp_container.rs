extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() {
    let mut container = MiniAppContainer::new();
    container.add_app("app1".into(), vec![1, 2, 3]);
    container.add_app("app2".into(), vec![4, 5, 6]);
    container.run_app("app1");
    container.list_apps();
    container.remove_app("app2");
    container.list_apps();
}

pub struct MiniAppContainer {
    apps: Vec<(String, Vec<u8>)>,
}

impl MiniAppContainer {
    pub fn new() -> Self {
        MiniAppContainer { apps: Vec::new() }
    }

    pub fn add_app(&mut self, name: String, data: Vec<u8>) {
        self.apps.push((name, data));
    }

    pub fn remove_app(&mut self, name: &str) {
        self.apps.retain(|(app_name, _)| app_name != name);
    }

    pub fn run_app(&self, name: &str) {
        if let Some((_, data)) = self.apps.iter().find(|(app_name, _)| app_name == name) {
            // Simulate running the app by printing its data
            for byte in data {
                // Placeholder for actual app execution logic
            }
        } else {
        }
    }

    pub fn list_apps(&self) {
        for (name, _) in &self.apps {
        }
    }

    pub fn get_app_data(&self, name: &str) -> Option<&Vec<u8>> {
        self.apps.iter().find(|(app_name, _)| app_name == name).map(|(_, data)| data)
    }
}
