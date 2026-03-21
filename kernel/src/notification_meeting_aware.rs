extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() {}

struct NotificationMeetingAware {
    notifications: Vec<String>,
    meetings: Vec<String>,
}

impl NotificationMeetingAware {
    pub fn new() -> Self {
        NotificationMeetingAware {
            notifications: Vec::new(),
            meetings: Vec::new(),
        }
    }

    pub fn add_notification(&mut self, notification: String) {
        self.notifications.push(notification);
    }

    pub fn remove_notification(&mut self, index: usize) -> Option<String> {
        if index < self.notifications.len() {
            Some(self.notifications.remove(index))
        } else {
            None
        }
    }

    pub fn add_meeting(&mut self, meeting: String) {
        self.meetings.push(meeting);
    }

    pub fn remove_meeting(&mut self, index: usize) -> Option<String> {
        if index < self.meetings.len() {
            Some(self.meetings.remove(index))
        } else {
            None
        }
    }

    pub fn list_notifications(&self) -> Vec<String> {
        self.notifications.clone()
    }

    pub fn list_meetings(&self) -> Vec<String> {
        self.meetings.clone()
    }
}

pub extern "C" fn create_notification_manager() -> *mut NotificationMeetingAware {
    Box::into_raw(Box::new(NotificationMeetingAware::new()))
}

pub extern "C" fn destroy_notification_manager(ptr: *mut NotificationMeetingAware) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) };
    }
}

pub extern "C" fn add_notification(
    ptr: *mut NotificationMeetingAware,
    notification: *const u8,
    len: usize,
 -> i32 {
    if ptr.is_null() || notification.is_null() {
        return -1;
    }

    let notification_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(notification, len)) };
    unsafe { (*ptr).add_notification(String::from(notification_str)); }
    0
}

pub extern "C" fn remove_notification(ptr: *mut NotificationMeetingAware, index: usize) -> i32 {
    if ptr.is_null() {
        return -1;
    }

    match unsafe { (*ptr).remove_notification(index) } {
        Some(_) => 0,
        None => -1,
    }
}

pub extern "C" fn add_meeting(
    ptr: *mut NotificationMeetingAware,
    meeting: *const u8,
    len: usize,
 -> i32 {
    if ptr.is_null() || meeting.is_null() {
        return -1;
    }

    let meeting_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(meeting, len)) };
    unsafe { (*ptr).add_meeting(String::from(meeting_str)); }
    0
}

pub extern "C" fn remove_meeting(ptr: *mut NotificationMeetingAware, index: usize) -> i32 {
    if ptr.is_null() {
        return -1;
    }

    match unsafe { (*ptr).remove_meeting(index) } {
        Some(_) => 0,
        None => -1,
    }
}

pub extern "C" fn list_notifications(
    ptr: *mut NotificationMeetingAware,
    notifications: *mut *const u8,
    lengths: *mut usize,
    count: *mut usize,
 -> i32 {
    if ptr.is_null() || notifications.is_null() || lengths.is_null() || count.is_null() {
        return -1;
    }

    let notification_manager = unsafe { &*ptr };
    let notifs = notification_manager.list_notifications();
    let num_notifs = notifs.len();

    unsafe {
        *count = num_notifs;
        for i in 0..num_notifs {
            (*notifications.offset(i as isize)) = notifs[i].as_ptr();
            (*lengths.offset(i as isize)) = notifs[i].len();
        }
    }

    0
}

pub extern "C" fn list_meetings(
    ptr: *mut NotificationMeetingAware,
    meetings: *mut *const u8,
    lengths: *mut usize,
    count: *mut usize,
 -> i32 {
    if ptr.is_null() || meetings.is_null() || lengths.is_null() || count.is_null() {
        return -1;
    }

    let notification_manager = unsafe { &*ptr };
    let meets = notification_manager.list_meetings();
    let num_meets = meets.len();

    unsafe {
        *count = num_meets;
        for i in 0..num_meets {
            (*meetings.offset(i as isize)) = meets[i].as_ptr();
            (*lengths.offset(i as isize)) = meets[i].len();
        }
    }

    0
}
