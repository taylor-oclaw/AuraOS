extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_std]
mod skill_permission_mgr {
    use core::cmp::Ordering;

    pub struct SkillPermissionManager {
        permissions: Vec<Permission>,
    }

    impl SkillPermissionManager {
        pub fn new() -> Self {
            SkillPermissionManager {
                permissions: Vec::new(),
            }
        }

        pub fn add_permission(&mut self, skill_name: String, permission_level: u8) {
            let permission = Permission {
                skill_name,
                permission_level,
            };
            self.permissions.push(permission);
        }

        pub fn remove_permission(&mut self, skill_name: &str) {
            self.permissions.retain(|p| p.skill_name != skill_name);
        }

        pub fn get_permission_level(&self, skill_name: &str) -> Option<u8> {
            self.permissions
                .iter()
                .find(|p| p.skill_name == skill_name)
                .map(|p| p.permission_level)
        }

        pub fn has_permission(&self, skill_name: &str, required_level: u8) -> bool {
            if let Some(level) = self.get_permission_level(skill_name) {
                level >= required_level
            } else {
                false
            }
        }

        pub fn list_permissions(&self) -> Vec<&Permission> {
            self.permissions.iter().collect()
        }
    }

    #[derive(Debug)]
    struct Permission {
        skill_name: String,
        permission_level: u8,
    }

    impl PartialEq for Permission {
        fn eq(&self, other: &Self) -> bool {
            self.skill_name == other.skill_name
        }
    }

    impl Eq for Permission {}

    impl PartialOrd for Permission {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Permission {
        fn cmp(&self, other: &Self) -> Ordering {
            self.skill_name.cmp(&other.skill_name)
        }
    }
}
