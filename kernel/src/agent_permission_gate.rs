extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentPermissionGate {
    permissions: Vec<String>,
}

impl AgentPermissionGate {
    pub fn new() -> Self {
        AgentPermissionGate {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_gate() {
        let mut gate = AgentPermissionGate::new();

        assert!(!gate.has_permission("read"));
        gate.add_permission(String::from("read"));
        assert!(gate.has_permission("read"));

        gate.add_permission(String::from("write"));
        assert!(gate.has_permission("write"));

        assert_eq!(gate.list_permissions(), vec![String::from("read"), String::from("write")]);

        assert!(gate.remove_permission("read"));
        assert!(!gate.has_permission("read"));

        gate.clear_permissions();
        assert_eq!(gate.list_permissions().len(), 0);
    }
}
