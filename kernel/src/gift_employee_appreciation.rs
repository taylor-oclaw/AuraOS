extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn gift_employee_birthday_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn gift_employee_birthday_exit() {
    // Cleanup logic for the module
}

pub struct Employee {
    name: String,
    birthday: String,
    gifts: Vec<String>,
}

impl Employee {
    pub fn new(name: &str, birthday: &str) -> Self {
        Employee {
            name: String::from(name),
            birthday: String::from(birthday),
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

    pub fn has_birthday_today(&self, today: &str) -> bool {
        self.birthday == today
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employee_methods() {
        let mut employee = Employee::new("Alice", "12/05");
        assert_eq!(employee.name, "Alice");
        assert_eq!(employee.birthday, "12/05");
        assert!(employee.gifts.is_empty());

        employee.add_gift("Book");
        employee.add_gift("Pen");
        assert_eq!(employee.list_gifts(), vec!["Book".to_string(), "Pen".to_string()]);

        assert!(employee.remove_gift("Pen"));
        assert_eq!(employee.list_gifts(), vec!["Book".to_string()]);
        assert!(!employee.remove_gift("Pen"));

        assert!(!employee.has_birthday_today("12/06"));
        assert!(employee.has_birthday_today("12/05"));
    }
}
