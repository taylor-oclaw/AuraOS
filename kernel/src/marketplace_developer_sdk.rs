extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn marketplace_developer_sdk_init() {
    // Initialization logic for the SDK
}

pub extern "C" fn marketplace_developer_sdk_exit() {
    // Cleanup logic for the SDK
}

pub struct MarketplaceDeveloperSDK {
    apps: Vec<String>,
    developers: Vec<String>,
}

impl MarketplaceDeveloperSDK {
    pub fn new() -> Self {
        MarketplaceDeveloperSDK {
            apps: Vec::new(),
            developers: Vec::new(),
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

    pub fn list_apps(&self) -> Vec<String> {
        self.apps.clone()
    }

    pub fn add_developer(&mut self, developer_name: &str) {
        self.developers.push(String::from(developer_name));
    }

    pub fn remove_developer(&mut self, developer_name: &str) -> bool {
        if let Some(index) = self.developers.iter().position(|x| x == developer_name) {
            self.developers.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_developers(&self) -> Vec<String> {
        self.developers.clone()
    }
}
