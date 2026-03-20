extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

struct Reminder {
    id: u32,
    message: String,
    is_completed: bool,
}

impl Reminder {
    pub fn new(id: u32, message: &str) -> Self {
        Reminder {
            id,
            message: String::from(message),
            is_completed: false,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn is_completed(&self) -> bool {
        self.is_completed
    }

    pub fn mark_as_completed(&mut self) {
        self.is_completed = true;
    }

    pub fn update_message(&mut self, new_message: &str) {
        self.message = String::from(new_message);
    }
}

struct AuraReminders {
    reminders: Vec<Reminder>,
}

impl AuraReminders {
    pub fn new() -> Self {
        AuraReminders {
            reminders: Vec::new(),
        }
    }

    pub fn add_reminder(&mut self, id: u32, message: &str) {
        let reminder = Reminder::new(id, message);
        self.reminders.push(reminder);
    }

    pub fn get_reminder(&self, id: u32) -> Option<&Reminder> {
        self.reminders.iter().find(|r| r.get_id() == id)
    }

    pub fn mark_reminder_as_completed(&mut self, id: u32) {
        if let Some(reminder) = self.reminders.iter_mut().find(|r| r.get_id() == id) {
            reminder.mark_as_completed();
        }
    }

    pub fn update_reminder_message(&mut self, id: u32, new_message: &str) {
        if let Some(reminder) = self.reminders.iter_mut().find(|r| r.get_id() == id) {
            reminder.update_message(new_message);
        }
    }

    pub fn list_reminders(&self) -> Vec<&Reminder> {
        self.reminders.iter().collect()
    }
}
