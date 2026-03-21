extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

mod family_hub_role_mgr {
    use super::*;

    pub struct RoleManager {
        roles: Vec<String>,
    }

    impl RoleManager {
        pub fn new() -> Self {
            RoleManager { roles: Vec::new() }
        }

        pub fn add_role(&mut self, role_name: &str) {
            if !self.roles.contains(&String::from(role_name)) {
                self.roles.push(String::from(role_name));
            }
        }

        pub fn remove_role(&mut self, role_name: &str) {
            self.roles.retain(|role| role != role_name);
        }

        pub fn has_role(&self, role_name: &str) -> bool {
            self.roles.contains(&String::from(role_name))
        }

        pub fn list_roles(&self) -> Vec<String> {
            self.roles.clone()
        }

        pub fn clear_roles(&mut self) {
            self.roles.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_manager() {
        let mut rm = family_hub_role_mgr::RoleManager::new();

        assert_eq!(rm.list_roles(), Vec::<String>::new());

        rm.add_role("admin");
        assert_eq!(rm.has_role("admin"), true);
        assert_eq!(rm.list_roles(), vec![String::from("admin")]);

        rm.add_role("user");
        assert_eq!(rm.has_role("user"), true);
        assert_eq!(rm.list_roles(), vec![String::from("admin"), String::from("user")]);

        rm.remove_role("admin");
        assert_eq!(rm.has_role("admin"), false);
        assert_eq!(rm.list_roles(), vec![String::from("user")]);

        rm.clear_roles();
        assert_eq!(rm.has_role("user"), false);
        assert_eq!(rm.list_roles(), Vec::<String>::new());
    }
}
