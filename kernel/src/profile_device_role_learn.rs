extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct ProfileDeviceRoleLearn {
    device_name: String,
    roles: Vec<String>,
    active_role_index: usize,
}

impl ProfileDeviceRoleLearn {
    pub fn new(device_name: &str) -> Self {
        ProfileDeviceRoleLearn {
            device_name: String::from(device_name),
            roles: Vec::new(),
            active_role_index: 0,
        }
    }

    pub fn add_role(&mut self, role: &str) {
        self.roles.push(String::from(role));
    }

    pub fn remove_role(&mut self, role: &str) -> bool {
        if let Some(index) = self.roles.iter().position(|r| r == role) {
            self.roles.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_active_role(&self) -> Option<&String> {
        self.roles.get(self.active_role_index)
    }

    pub fn switch_to_next_role(&mut self) {
        if !self.roles.is_empty() {
            self.active_role_index = (self.active_role_index + 1) % self.roles.len();
        }
    }

    pub fn list_roles(&self) -> Vec<&String> {
        self.roles.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_device_role_learn() {
        let mut profile = ProfileDeviceRoleLearn::new("TestDevice");
        assert_eq!(profile.device_name, "TestDevice");
        assert!(profile.list_roles().is_empty());

        profile.add_role("Role1");
        profile.add_role("Role2");
        assert_eq!(profile.list_roles(), vec!["Role1", "Role2"]);
        assert_eq!(profile.get_active_role(), Some(&String::from("Role1")));

        profile.switch_to_next_role();
        assert_eq!(profile.get_active_role(), Some(&String::from("Role2")));

        profile.switch_to_next_role();
        assert_eq!(profile.get_active_role(), Some(&String::from("Role1")));

        assert!(profile.remove_role("Role1"));
        assert!(!profile.list_roles().contains(&String::from("Role1")));
        assert_eq!(profile.list_roles(), vec!["Role2"]);
    }
}
