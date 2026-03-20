extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialization code for the module
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup code for the module
}

pub struct AuraAgentPermissionUI {
    permissions: Vec<String>,
}

impl AuraAgentPermissionUI {
    pub fn new() -> Self {
        AuraAgentPermissionUI {
            permissions: Vec::new(),
        }
    }

    pub fn add_permission(&mut self, permission: &str) {
        if !self.permissions.contains(&String::from(permission)) {
            self.permissions.push(String::from(permission));
        }
    }

    pub fn remove_permission(&mut self, permission: &str) {
        self.permissions.retain(|p| p != permission);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_ui() {
        let mut ui = AuraAgentPermissionUI::new();

        assert_eq!(ui.list_permissions(), Vec::<String>::new());

        ui.add_permission("read");
        ui.add_permission("write");

        assert_eq!(ui.has_permission("read"), true);
        assert_eq!(ui.has_permission("write"), true);
        assert_eq!(ui.has_permission("execute"), false);

        let permissions = ui.list_permissions();
        assert_eq!(permissions.len(), 2);
        assert!(permissions.contains(&String::from("read")));
        assert!(permissions.contains(&String::from("write")));

        ui.remove_permission("read");
        assert_eq!(ui.has_permission("read"), false);

        ui.clear_permissions();
        assert_eq!(ui.list_permissions(), Vec::<String>::new());
    }
}
