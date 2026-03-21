extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct CrossDeviceBrowserTabSync {
    tabs: Vec<String>,
    device_id: String,
}

impl CrossDeviceBrowserTabSync {
    pub fn new(device_id: &str) -> Self {
        CrossDeviceBrowserTabSync {
            tabs: Vec::new(),
            device_id: String::from(device_id),
        }
    }

    pub fn add_tab(&mut self, tab_url: &str) {
        self.tabs.push(String::from(tab_url));
    }

    pub fn remove_tab(&mut self, tab_index: usize) -> Option<String> {
        if tab_index < self.tabs.len() {
            Some(self.tabs.remove(tab_index))
        } else {
            None
        }
    }

    pub fn get_tabs(&self) -> &Vec<String> {
        &self.tabs
    }

    pub fn sync_tabs(&mut self, other_device_tabs: Vec<&str>) {
        for tab_url in other_device_tabs {
            if !self.tabs.contains(&String::from(tab_url)) {
                self.add_tab(tab_url);
            }
        }
    }

    pub fn get_device_id(&self) -> &str {
        &self.device_id
    }
}
