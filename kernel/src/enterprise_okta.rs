extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut okta = EnterpriseOkta::new();
    okta.add_user("alice".into(), "admin".into());
    okta.add_user("bob".into(), "user".into());
    okta.remove_user("alice".into());
    if okta.authenticate("bob", "user") {
        // Authentication successful
    } else {
        // Authentication failed
    }
    loop {}
}

pub struct EnterpriseOkta {
    users: Vec<User>,
}

impl EnterpriseOkta {
    pub fn new() -> Self {
        EnterpriseOkta { users: Vec::new() }
    }

    pub fn add_user(&mut self, username: String, role: String) {
        let user = User { username, role };
        self.users.push(user);
    }

    pub fn remove_user(&mut self, username: String) -> bool {
        self.users.retain(|u| u.username != username);
        !self.users.iter().any(|u| u.username == username)
    }

    pub fn authenticate(&self, username: &str, password: &str) -> bool {
        self.users
            .iter()
            .any(|u| u.username == username && u.password == password)
    }

    pub fn get_user_role(&self, username: &str) -> Option<&String> {
        self.users.iter().find_map(|u| {
            if u.username == username {
                Some(&u.role)
            } else {
                None
            }
        })
    }

    pub fn list_users(&self) -> Vec<String> {
        self.users.iter().map(|u| u.username.clone()).collect()
    }
}

struct User {
    username: String,
    role: String,
    password: String, // In a real scenario, this should be hashed
}

impl User {
    fn new(username: String, role: String, password: String) -> Self {
        User { username, role, password }
    }
}
