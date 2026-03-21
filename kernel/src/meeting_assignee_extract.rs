extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn meeting_assignee_extract_init() {
    // Initialization code for the module
}

pub extern "C" fn meeting_assignee_extract_exit() {
    // Cleanup code for the module
}

pub struct MeetingAssigneeExtractor {
    assignees: Vec<String>,
}

impl MeetingAssigneeExtractor {
    pub fn new() -> Self {
        MeetingAssigneeExtractor {
            assignees: Vec::new(),
        }
    }

    pub fn add_assignee(&mut self, name: &str) {
        self.assignees.push(String::from(name));
    }

    pub fn remove_assignee(&mut self, name: &str) {
        if let Some(index) = self.assignees.iter().position(|n| n == name) {
            self.assignees.remove(index);
        }
    }

    pub fn get_assignees(&self) -> Vec<String> {
        self.assignees.clone()
    }

    pub fn has_assignee(&self, name: &str) -> bool {
        self.assignees.contains(&String::from(name))
    }

    pub fn clear_assignees(&mut self) {
        self.assignees.clear();
    }
}

pub extern "C" fn meeting_assignee_extractor_new() -> *mut MeetingAssigneeExtractor {
    Box::into_raw(Box::new(MeetingAssigneeExtractor::new()))
}

pub extern "C" fn meeting_assignee_extractor_add_assignee(me: *mut MeetingAssigneeExtractor, name: *const u8, len: usize) {
    unsafe {
        let extractor = &mut *me;
        let name_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(name, len));
        extractor.add_assignee(name_str);
    }
}

pub extern "C" fn meeting_assignee_extractor_remove_assignee(me: *mut MeetingAssigneeExtractor, name: *const u8, len: usize) {
    unsafe {
        let extractor = &mut *me;
        let name_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(name, len));
        extractor.remove_assignee(name_str);
    }
}

pub extern "C" fn meeting_assignee_extractor_get_assignees(me: *const MeetingAssigneeExtractor) -> Vec<String> {
    unsafe {
        let extractor = &*me;
        extractor.get_assignees()
    }
}

pub extern "C" fn meeting_assignee_extractor_has_assignee(me: *const MeetingAssigneeExtractor, name: *const u8, len: usize) -> bool {
    unsafe {
        let extractor = &*me;
        let name_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(name, len));
        extractor.has_assignee(name_str)
    }
}

pub extern "C" fn meeting_assignee_extractor_clear_assignees(me: *mut MeetingAssigneeExtractor) {
    unsafe {
        let extractor = &mut *me;
        extractor.clear_assignees();
    }
}
