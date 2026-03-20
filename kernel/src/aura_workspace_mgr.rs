extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

pub struct AuraWorkspaceMgr {
    workspaces: Vec<String>,
    current_workspace_index: usize,
}

impl AuraWorkspaceMgr {
    pub fn new() -> Self {
        AuraWorkspaceMgr {
            workspaces: Vec::new(),
            current_workspace_index: 0,
        }
    }

    pub fn add_workspace(&mut self, name: &str) {
        self.workspaces.push(String::from(name));
        if self.current_workspace_index == 0 {
            self.current_workspace_index = 1;
        }
    }

    pub fn remove_workspace(&mut self, index: usize) -> Option<String> {
        if index < self.workspaces.len() {
            let removed = self.workspaces.remove(index);
            if self.current_workspace_index > self.workspaces.len() {
                self.current_workspace_index = self.workspaces.len();
            }
            Some(removed)
        } else {
            None
        }
    }

    pub fn switch_to_workspace(&mut self, index: usize) -> bool {
        if index < self.workspaces.len() {
            self.current_workspace_index = index + 1;
            true
        } else {
            false
        }
    }

    pub fn get_current_workspace(&self) -> Option<&String> {
        if self.current_workspace_index > 0 && self.current_workspace_index <= self.workspaces.len() {
            Some(&self.workspaces[self.current_workspace_index - 1])
        } else {
            None
        }
    }

    pub fn list_workspaces(&self) -> Vec<String> {
        self.workspaces.clone()
    }
}
