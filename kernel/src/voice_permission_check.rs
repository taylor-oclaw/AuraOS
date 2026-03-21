extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoicePermissionCheck {
    permissions: Vec<String>,
}

impl VoicePermissionCheck {
    pub fn new() -> Self {
        VoicePermissionCheck {
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
