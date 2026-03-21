extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn notification_sleeping_aware_init() {
    // Initialization logic for the module
}

pub extern "C" fn notification_sleeping_aware_exit() {
    // Cleanup logic for the module
}

pub struct NotificationSleepingAware {
    notifications: Vec<String>,
    sleeping: bool,
}

impl NotificationSleepingAware {
    pub fn new() -> Self {
        NotificationSleepingAware {
            notifications: Vec::new(),
            sleeping: false,
        }
    }

    pub fn add_notification(&mut self, notification: String) {
        if !self.sleeping {
            self.notifications.push(notification);
        }
    }

    pub fn get_notifications(&self) -> &[String] {
        &self.notifications
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub fn set_sleeping(&mut self, sleeping: bool) {
        self.sleeping = sleeping;
    }

    pub fn is_sleeping(&self) -> bool {
        self.sleeping
    }
}

pub extern "C" fn notification_sleeping_aware_add_notification(
    module: *mut NotificationSleepingAware,
    notification: *const u8,
    len: usize,
 {
    unsafe {
        if let Some(module) = module.as_mut() {
            let notification_str = String::from_utf8_lossy(slice::from_raw_parts(notification, len)).into_owned();
            module.add_notification(notification_str);
        }
    }
}

pub extern "C" fn notification_sleeping_aware_get_notifications(
    module: *const NotificationSleepingAware,
 -> *const Vec<String> {
    unsafe {
        if let Some(module) = module.as_ref() {
            module.get_notifications() as *const _
        } else {
            core::ptr::null()
        }
    }
}

pub extern "C" fn notification_sleeping_aware_clear_notifications(
    module: *mut NotificationSleepingAware,
 {
    unsafe {
        if let Some(module) = module.as_mut() {
            module.clear_notifications();
        }
    }
}

pub extern "C" fn notification_sleeping_aware_set_sleeping(
    module: *mut NotificationSleepingAware,
    sleeping: bool,
 {
    unsafe {
        if let Some(module) = module.as_mut() {
            module.set_sleeping(sleeping);
        }
    }
}

pub extern "C" fn notification_sleeping_aware_is_sleeping(
    module: *const NotificationSleepingAware,
 -> bool {
    unsafe {
        if let Some(module) = module.as_ref() {
            module.is_sleeping()
        } else {
            false
        }
    }
}
