extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn gift_client_holiday_init() {
    // Initialization logic for the module
}

pub extern "C" fn gift_client_holiday_exit() {
    // Cleanup logic for the module
}

pub struct GiftClientHoliday {
    gifts: Vec<String>,
    recipients: Vec<String>,
}

impl GiftClientHoliday {
    pub fn new() -> Self {
        GiftClientHoliday {
            gifts: Vec::new(),
            recipients: Vec::new(),
        }
    }

    pub fn add_gift(&mut self, gift: String) {
        self.gifts.push(gift);
    }

    pub fn add_recipient(&mut self, recipient: String) {
        self.recipients.push(recipient);
    }

    pub fn list_gifts(&self) -> Vec<String> {
        self.gifts.clone()
    }

    pub fn list_recipients(&self) -> Vec<String> {
        self.recipients.clone()
    }

    pub fn assign_gifts(&mut self) -> Option<Vec<(String, String)>> {
        if self.gifts.is_empty() || self.recipients.is_empty() {
            return None;
        }

        let mut assignments = Vec::new();
        for recipient in &self.recipients {
            if let Some(gift) = self.gifts.pop() {
                assignments.push((recipient.clone(), gift));
            }
        }

        Some(assignments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gift_client_holiday() {
        let mut holiday = GiftClientHoliday::new();
        holiday.add_gift(String::from("Book"));
        holiday.add_gift(String::from("Toy"));
        holiday.add_recipient(String::from("Alice"));
        holiday.add_recipient(String::from("Bob"));

        assert_eq!(holiday.list_gifts(), vec![String::from("Book"), String::from("Toy")]);
        assert_eq!(holiday.list_recipients(), vec![String::from("Alice"), String::from("Bob")]);

        let assignments = holiday.assign_gifts();
        assert!(assignments.is_some());
        let assignments = assignments.unwrap();
        assert_eq!(assignments.len(), 2);
    }
}
