extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn gift_employee_welcome_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn gift_employee_welcome_exit() {
    // Cleanup code for the module
}

pub struct EmployeeWelcome {
    name: String,
    department: String,
    gifts: Vec<String>,
}

impl EmployeeWelcome {
    pub fn new(name: &str, department: &str) -> Self {
        EmployeeWelcome {
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

    pub fn welcome_message(&self) -> String {
        let mut message = String::from("Welcome, ");
        message.push_str(&self.name);
        message.push_str("! You are part of the ");
        message.push_str(&self.department);
        message.push_str(" department. Here are your gifts: ");

        for (i, gift) in self.gifts.iter().enumerate() {
            if i > 0 {
                message.push_str(", ");
            }
            message.push_str(gift);
        }

        message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employee_welcome() {
        let mut ew = EmployeeWelcome::new("Alice", "Engineering");
        ew.add_gift("Laptop");
        ew.add_gift("Mouse");

        assert_eq!(ew.list_gifts(), vec![String::from("Laptop"), String::from("Mouse")]);
        assert!(ew.remove_gift("Mouse"));
        assert!(!ew.remove_gift("Keyboard"));
        assert_eq!(ew.welcome_message(), "Welcome, Alice! You are part of the Engineering department. Here are your gifts: Laptop");
    }
}
