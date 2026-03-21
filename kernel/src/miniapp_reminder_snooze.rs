extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn miniapp_reminder_snooze_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn miniapp_reminder_snooze_exit() {
    // Cleanup logic for the module
}

pub struct ReminderSnooze {
    reminders: Vec<String>,
    snoozed: bool,
}

impl ReminderSnooze {
    pub fn new() -> Self {
        ReminderSnooze {
            reminders: Vec::new(),
            snoozed: false,
        }
    }

    pub fn add_reminder(&mut self, reminder: String) {
        self.reminders.push(reminder);
    }

    pub fn remove_reminder(&mut self, index: usize) -> Option<String> {
        if index < self.reminders.len() {
            Some(self.reminders.remove(index))
        } else {
            None
        }
    }

    pub fn list_reminders(&self) -> &Vec<String> {
        &self.reminders
    }

    pub fn snooze(&mut self) {
        self.snoozed = true;
    }

    pub fn unsnooze(&mut self) {
        self.snoozed = false;
    }

    pub fn is_snoozed(&self) -> bool {
        self.snoozed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reminder_snooze() {
        let mut reminder_snooze = ReminderSnooze::new();
        assert_eq!(reminder_snooze.list_reminders().len(), 0);
        assert!(!reminder_snooze.is_snoozed());

        reminder_snooze.add_reminder(String::from("Buy groceries"));
        assert_eq!(reminder_snooze.list_reminders().len(), 1);

        let removed = reminder_snooze.remove_reminder(0);
        assert_eq!(removed, Some(String::from("Buy groceries")));
        assert_eq!(reminder_snooze.list_reminders().len(), 0);

        reminder_snooze.snooze();
        assert!(reminder_snooze.is_snoozed());

        reminder_snooze.unsnooze();
        assert!(!reminder_snooze.is_snoozed());
    }
}
