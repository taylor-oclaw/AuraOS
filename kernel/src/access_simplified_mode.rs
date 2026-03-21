extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AccessSimplifiedMode {
    enabled: bool,
    users: Vec<String>,
    permissions: Vec<String>,
}

impl AccessSimplifiedMode {
    pub fn new() -> Self {
        AccessSimplifiedMode {
            enabled: false,
            users: Vec::new(),
            permissions: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn add_user(&mut self, user: String) {
        if !self.users.contains(&user) {
            self.users.push(user);
        }
    }

    pub fn remove_user(&mut self, user: &str) {
        self.users.retain(|u| u != user);
    }

    pub fn grant_permission(&mut self, permission: String) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
        }
    }

    pub fn revoke_permission(&mut self, permission: &str) {
        self.permissions.retain(|p| p != permission);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn list_users(&self) -> Vec<String> {
        self.users.clone()
    }

    pub fn list_permissions(&self) -> Vec<String> {
        self.permissions.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_simplified_mode() {
        let mut mode = AccessSimplifiedMode::new();
        assert!(!mode.is_enabled());

        mode.enable();
        assert!(mode.is_enabled());

        mode.disable();
        assert!(!mode.is_enabled());

        mode.add_user(String::from("user1"));
        assert_eq!(mode.list_users(), vec![String::from("user1")]);

        mode.remove_user("user1");
        assert_eq!(mode.list_users(), Vec::<String>::new());

        mode.grant_permission(String::from("read"));
        assert_eq!(mode.list_permissions(), vec![String::from("read")]);

        mode.revoke_permission("read");
        assert_eq!(mode.list_permissions(), Vec::<String>::new());
    }
}
