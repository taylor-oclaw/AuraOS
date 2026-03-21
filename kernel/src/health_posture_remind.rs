extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn health_posture_remind_init() {
    // Initialization logic for the module
}

pub extern "C" fn health_posture_remind_exit() {
    // Cleanup logic for the module
}

pub struct HealthPostureRemind {
    reminders: Vec<String>,
    current_index: usize,
}

impl HealthPostureRemind {
    pub fn new() -> Self {
        let mut reminders = Vec::new();
        reminders.push(String::from("Stand up and stretch every 30 minutes."));
        reminders.push(String::from("Take a short walk to refresh your mind."));
        reminders.push(String::from("Adjust your screen height for better posture."));
        reminders.push(String::from("Drink water to stay hydrated."));
        reminders.push(String::from("Do some neck and shoulder exercises."));

        HealthPostureRemind {
            reminders,
            current_index: 0,
        }
    }

    pub fn get_next_reminder(&mut self) -> Option<&str> {
        if let Some(reminder) = self.reminders.get(self.current_index) {
            self.current_index = (self.current_index + 1) % self.reminders.len();
            Some(reminder)
        } else {
            None
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

    pub fn list_reminders(&self) -> Vec<&str> {
        self.reminders.iter().map(|r| r.as_str()).collect()
    }
}

pub extern "C" fn health_posture_remind_get_next() -> *const u8 {
    let mut hpr = HealthPostureRemind::new();
    match hpr.get_next_reminder() {
        Some(reminder) => reminder.as_ptr(),
        None => core::ptr::null(),
    }
}

pub extern "C" fn health_posture_remind_add(reminder: *const u8, len: usize) -> i32 {
    let mut hpr = HealthPostureRemind::new();
    if reminder.is_null() || len == 0 {
        return -1;
    }
    let c_str = unsafe { core::slice::from_raw_parts(reminder, len) };
    if let Ok(s) = core::str::from_utf8(c_str) {
        hpr.add_reminder(String::from(s));
        0
    } else {
        -1
    }
}

pub extern "C" fn health_posture_remind_remove(index: usize) -> i32 {
    let mut hpr = HealthPostureRemind::new();
    if hpr.remove_reminder(index).is_some() {
        0
    } else {
        -1
    }
}

pub extern "C" fn health_posture_remind_list(count: *mut usize) -> *const *const u8 {
    let hpr = HealthPostureRemind::new();
    let reminders = hpr.list_reminders();
    let mut ptrs: Vec<*const u8> = reminders.iter().map(|r| r.as_ptr()).collect();
    unsafe { *count = ptrs.len() };
    ptrs.as_ptr()
}
