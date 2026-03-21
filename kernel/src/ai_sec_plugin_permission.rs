extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct PermissionManager {
    permissions: Vec<String>,
}

impl PermissionManager {
    pub fn new() -> Self {
        PermissionManager {
            permissions: Vec::new(),
        }
    }

    pub fn add_permission(&mut self, permission: String) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
        }
    }

    pub fn remove_permission(&mut self, permission: &str) -> bool {
        let index = self.permissions.iter().position(|p| p == permission);
        match index {
            Some(i) => {
                self.permissions.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&String::from(permission))
    }

    pub fn list_permissions(&self) -> Vec<String> {
        self.permissions.clone()
    }

    pub fn clear_permissions(&mut self) {
        self.permissions.clear();
    }
}
