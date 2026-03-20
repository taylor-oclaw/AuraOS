extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct OrgRoleMgr {
    roles: Vec<String>,
}

impl OrgRoleMgr {
    pub fn new() -> Self {
        OrgRoleMgr { roles: Vec::new() }
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

    pub fn count_roles(&self) -> usize {
        self.roles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_org_role_mgr() {
        let mut mgr = OrgRoleMgr::new();
        assert_eq!(mgr.count_roles(), 0);

        mgr.add_role("Admin");
        assert_eq!(mgr.count_roles(), 1);
        assert!(mgr.has_role("Admin"));
        assert!(!mgr.has_role("User"));

        mgr.add_role("User");
        assert_eq!(mgr.count_roles(), 2);
        assert!(mgr.has_role("User"));

        let roles = mgr.list_roles();
        assert_eq!(roles.len(), 2);
        assert!(roles.contains(&String::from("Admin")));
        assert!(roles.contains(&String::from("User")));

        mgr.remove_role("Admin");
        assert_eq!(mgr.count_roles(), 1);
        assert!(!mgr.has_role("Admin"));
        assert!(mgr.has_role("User"));

        mgr.remove_role("User");
        assert_eq!(mgr.count_roles(), 0);
        assert!(!mgr.has_role("User"));
    }
}
