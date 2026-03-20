extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

pub struct AuraDisplayMgr {
    displays: Vec<String>,
    active_display: Option<usize>,
}

impl AuraDisplayMgr {
    pub fn new() -> Self {
        AuraDisplayMgr {
            displays: Vec::new(),
            active_display: None,
        }
    }

    pub fn add_display(&mut self, display_name: &str) {
        let name = String::from(display_name);
        self.displays.push(name);
    }

    pub fn remove_display(&mut self, index: usize) -> Option<String> {
        if index < self.displays.len() {
            Some(self.displays.remove(index))
        } else {
            None
        }
    }

    pub fn set_active_display(&mut self, index: usize) -> bool {
        if index < self.displays.len() {
            self.active_display = Some(index);
            true
        } else {
            false
        }
    }

    pub fn get_active_display(&self) -> Option<&String> {
        self.active_display.map(|index| &self.displays[index])
    }

    pub fn list_displays(&self) -> Vec<&String> {
        self.displays.iter().collect()
    }
}
