extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EnterpriseRBAC {
    users: Vec<User>,
    roles: Vec<Role>,
}

impl EnterpriseRBAC {
    pub fn new() -> Self {
        EnterpriseRBAC {
            users: Vec::new(),
            roles: Vec::new(),
        }
    }

    pub fn add_user(&mut self, username: String, role_id: usize) -> Result<(), &'static str> {
        if let Some(role) = self.roles.get(role_id) {
            self.users.push(User { username, role });
            Ok(())
        } else {
            Err("Role not found")
        }
    }

    pub fn add_role(&mut self, name: String, permissions: Vec<String>) -> usize {
        let role_id = self.roles.len();
        self.roles.push(Role { name, permissions });
        role_id
    }

    pub fn get_user_permissions(&self, username: &str) -> Option<&Vec<String>> {
        self.users.iter()
            .find(|user| user.username == username)
            .map(|user| &user.role.permissions)
    }

    pub fn check_permission(&self, username: &str, permission: &str) -> bool {
        if let Some(permissions) = self.get_user_permissions(username) {
            permissions.contains(&permission.to_string())
        } else {
            false
        }
    }

    pub fn list_users_by_role(&self, role_id: usize) -> Vec<&String> {
        self.users.iter()
            .filter(|user| user.role.id == role_id)
            .map(|user| &user.username)
            .collect()
    }
}

struct User {
    username: String,
    role: Role,
}

struct Role {
    name: String,
    permissions: Vec<String>,
    id: usize,
}
