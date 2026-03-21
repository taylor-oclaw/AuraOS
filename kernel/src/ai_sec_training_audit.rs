extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut audit = AITrainingAudit::new();
    audit.log_event(String::from("Module initialized"));
    audit.add_user(String::from("admin"), String::from("securepassword123"));
    audit.authenticate_user("admin", "securepassword123");
    audit.list_users();
    audit.clear_logs();
    loop {}
}

pub struct AITrainingAudit {
    users: Vec<(String, String)>,
    logs: Vec<String>,
}

impl AITrainingAudit {
    pub fn new() -> Self {
        AITrainingAudit {
            users: Vec::new(),
            logs: Vec::new(),
        }
    }

    pub fn add_user(&mut self, username: String, password: String) {
        self.users.push((username, password));
        self.log_event(format!("User added: {}", &self.users.last().unwrap().0));
    }

    pub fn authenticate_user(&self, username: &str, password: &str) -> bool {
        let result = self.users.iter().any(|(u, p)| u == username && p == password);
        if result {
            self.log_event(format!("User authenticated: {}", username));
        } else {
            self.log_event(format!("Failed authentication attempt for user: {}", username));
        }
        result
    }

    pub fn list_users(&self) {
        for (username, _) in &self.users {
            self.log_event(format!("Listing user: {}", username));
        }
    }

    pub fn log_event(&mut self, event: String) {
        self.logs.push(event);
    }

    pub fn clear_logs(&mut self) {
        self.logs.clear();
        self.log_event(String::from("Logs cleared"));
    }
}
