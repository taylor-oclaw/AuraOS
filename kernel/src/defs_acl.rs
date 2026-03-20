extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

mod defs_acl {
    use super::*;

    pub struct AclEntry {
        user: String,
        permissions: Vec<String>,
    }

    impl AclEntry {
        pub fn new(user: &str, permissions: &[&str]) -> Self {
            AclEntry {
                user: String::from(user),
                permissions: permissions.iter().map(|&perm| String::from(perm)).collect(),
            }
        }

        pub fn add_permission(&mut self, permission: &str) {
            if !self.permissions.contains(&String::from(permission)) {
                self.permissions.push(String::from(permission));
            }
        }

        pub fn remove_permission(&mut self, permission: &str) {
            self.permissions.retain(|perm| perm != permission);
        }

        pub fn has_permission(&self, permission: &str) -> bool {
            self.permissions.contains(&String::from(permission))
        }

        pub fn list_permissions(&self) -> Vec<String> {
            self.permissions.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acl_entry() {
        let mut entry = defs_acl::AclEntry::new("user1", &["read", "write"]);
        assert!(entry.has_permission("read"));
        assert!(entry.has_permission("write"));
        assert!(!entry.has_permission("execute"));

        entry.add_permission("execute");
        assert!(entry.has_permission("execute"));

        entry.remove_permission("write");
        assert!(!entry.has_permission("write"));

        let permissions = entry.list_permissions();
        assert_eq!(permissions, vec!["read", "execute"]);
    }
}
