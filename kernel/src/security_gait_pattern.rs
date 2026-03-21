extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn security_gait_pattern_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn security_gait_pattern_exit() {
    // Cleanup logic for the module
}

pub struct GaitPattern {
    steps: Vec<String>,
    pattern_length: usize,
}

impl GaitPattern {
    pub fn new(pattern_length: usize) -> Self {
        GaitPattern {
            steps: Vec::new(),
            pattern_length,
        }
    }

    pub fn add_step(&mut self, step: String) {
        if self.steps.len() < self.pattern_length {
            self.steps.push(step);
        }
    }

    pub fn remove_step(&mut self, index: usize) -> Option<String> {
        if index < self.steps.len() {
            Some(self.steps.remove(index))
        } else {
            None
        }
    }

    pub fn get_pattern(&self) -> Vec<&String> {
        self.steps.iter().collect()
    }

    pub fn is_complete(&self) -> bool {
        self.steps.len() == self.pattern_length
    }

    pub fn clear_pattern(&mut self) {
        self.steps.clear();
    }
}

#[no_mangle]
pub extern "C" fn security_gait_pattern_add_step(pattern: *mut GaitPattern, step: *const u8, len: usize) -> i32 {
    if pattern.is_null() || step.is_null() || len == 0 {
        return -1;
    }

    let pattern = unsafe { &mut *pattern };
    let step_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(step, len)) };
    pattern.add_step(String::from(step_str));
    0
}

#[no_mangle]
pub extern "C" fn security_gait_pattern_remove_step(pattern: *mut GaitPattern, index: usize) -> i32 {
    if pattern.is_null() || index == 0 {
        return -1;
    }

    let pattern = unsafe { &mut *pattern };
    pattern.remove_step(index);
    0
}

#[no_mangle]
pub extern "C" fn security_gait_pattern_get_pattern(pattern: *const GaitPattern, buffer: *mut u8, len: usize) -> i32 {
    if pattern.is_null() || buffer.is_null() || len == 0 {
        return -1;
    }

    let pattern = unsafe { &*pattern };
    let steps_str = pattern.get_pattern().iter().fold(String::new(), |acc, step| acc + step);
    let bytes_to_copy = core::cmp::min(steps_str.len(), len);
    unsafe {
        core::ptr::copy_nonoverlapping(steps_str.as_ptr(), buffer, bytes_to_copy);
    }
    bytes_to_copy as i32
}

#[no_mangle]
pub extern "C" fn security_gait_pattern_is_complete(pattern: *const GaitPattern) -> bool {
    if pattern.is_null() {
        return false;
    }

    let pattern = unsafe { &*pattern };
    pattern.is_complete()
}

#[no_mangle]
pub extern "C" fn security_gait_pattern_clear_pattern(pattern: *mut GaitPattern) {
    if !pattern.is_null() {
        let pattern = unsafe { &mut *pattern };
        pattern.clear_pattern();
    }
}
