extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_payment_remind_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_payment_remind_exit() {
    // Cleanup logic for the module
}

pub struct PaymentReminder {
    reminders: Vec<String>,
}

impl PaymentReminder {
    pub fn new() -> Self {
        PaymentReminder {
            reminders: Vec::new(),
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

    pub fn get_reminder(&self, index: usize) -> Option<&String> {
        self.reminders.get(index)
    }

    pub fn list_all_reminders(&self) -> Vec<String> {
        self.reminders.clone()
    }

    pub fn clear_all_reminders(&mut self) {
        self.reminders.clear();
    }
}

pub extern "C" fn rel_payment_remind_add(reminder: *const u8, len: usize) -> i32 {
    if reminder.is_null() || len == 0 {
        return -1;
    }

    let reminder_str = unsafe { core::slice::from_raw_parts(reminder, len) };
    let reminder_string = String::from_utf8_lossy(reminder_str).into_owned();

    // Assuming there's a global instance of PaymentReminder
    static mut PAYMENT_REMINDER: Option<PaymentReminder> = None;

    unsafe {
        if let Some(ref mut pr) = PAYMENT_REMINDER {
            pr.add_reminder(reminder_string);
        } else {
            PAYMENT_REMINDER = Some(PaymentReminder::new());
            PAYMENT_REMINDER.as_mut().unwrap().add_reminder(reminder_string);
        }
    }

    0
}

pub extern "C" fn rel_payment_remind_remove(index: usize) -> i32 {
    // Assuming there's a global instance of PaymentReminder
    static mut PAYMENT_REMINDER: Option<PaymentReminder> = None;

    unsafe {
        if let Some(ref mut pr) = PAYMENT_REMINDER {
            if pr.remove_reminder(index).is_some() {
                0
            } else {
                -1
            }
        } else {
            -1
        }
    }
}

pub extern "C" fn rel_payment_remind_get(index: usize, buffer: *mut u8, len: usize) -> i32 {
    if buffer.is_null() || len == 0 {
        return -1;
    }

    // Assuming there's a global instance of PaymentReminder
    static mut PAYMENT_REMINDER: Option<PaymentReminder> = None;

    unsafe {
        if let Some(ref pr) = PAYMENT_REMINDER {
            if let Some(reminder) = pr.get_reminder(index) {
                let reminder_bytes = reminder.as_bytes();
                if reminder_bytes.len() <= len {
                    core::ptr::copy_nonoverlapping(reminder_bytes.as_ptr(), buffer, reminder_bytes.len());
                    0
                } else {
                    -1
                }
            } else {
                -1
            }
        } else {
            -1
        }
    }
}

pub extern "C" fn rel_payment_remind_list(buffer: *mut u8, len: usize) -> i32 {
    if buffer.is_null() || len == 0 {
        return -1;
    }

    // Assuming there's a global instance of PaymentReminder
    static mut PAYMENT_REMINDER: Option<PaymentReminder> = None;

    unsafe {
        if let Some(ref pr) = PAYMENT_REMINDER {
            let reminders_str = pr.list_all_reminders().join(", ");
            let reminder_bytes = reminders_str.as_bytes();
            if reminder_bytes.len() <= len {
                core::ptr::copy_nonoverlapping(reminder_bytes.as_ptr(), buffer, reminder_bytes.len());
                0
            } else {
                -1
            }
        } else {
            -1
        }
    }
}

pub extern "C" fn rel_payment_remind_clear() -> i32 {
    // Assuming there's a global instance of PaymentReminder
    static mut PAYMENT_REMINDER: Option<PaymentReminder> = None;

    unsafe {
        if let Some(ref mut pr) = PAYMENT_REMINDER {
            pr.clear_all_reminders();
            0
        } else {
            -1
        }
    }
}
