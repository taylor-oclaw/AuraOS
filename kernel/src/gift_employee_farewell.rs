extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn gift_employee_farewell_init() {
    // Initialization logic for the module
}

pub extern "C" fn gift_employee_farewell_exit() {
    // Cleanup logic for the module
}

pub struct EmployeeFarewell {
    name: String,
    department: String,
    gifts: Vec<String>,
}

impl EmployeeFarewell {
    pub fn new(name: &str, department: &str) -> Self {
        EmployeeFarewell {
            name: String::from(name),
            department: String::from(department),
            gifts: Vec::new(),
        }
    }

    pub fn add_gift(&mut self, gift: &str) {
        self.gifts.push(String::from(gift));
    }

    pub fn remove_gift(&mut self, gift: &str) -> bool {
        if let Some(index) = self.gifts.iter().position(|g| g == gift) {
            self.gifts.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_gifts(&self) -> Vec<String> {
        self.gifts.clone()
    }

    pub fn get_department(&self) -> String {
        self.department.clone()
    }

    pub fn farewell_message(&self) -> String {
        let mut message = String::from("info");
        if !self.gifts.is_empty() {
            message.push_str("Here are your gifts:\n");
            for gift in &self.gifts {
                message.push_str(&String::from("info"));
            }
        } else {
            message.push_str("No gifts this time.\n");
        }
        message
    }
}
