extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_group_suggest_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rel_group_suggest_exit() {
    // Cleanup logic for the module
}

pub struct RelGroupSuggest {
    suggestions: Vec<String>,
}

impl RelGroupSuggest {
    pub fn new() -> Self {
        RelGroupSuggest {
            suggestions: Vec::new(),
        }
    }

    pub fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
    }

    pub fn remove_suggestion(&mut self, index: usize) -> Option<String> {
        if index < self.suggestions.len() {
            Some(self.suggestions.remove(index))
        } else {
            None
        }
    }

    pub fn get_suggestion(&self, index: usize) -> Option<&String> {
        self.suggestions.get(index)
    }

    pub fn list_suggestions(&self) -> &[String] {
        &self.suggestions
    }

    pub fn clear_suggestions(&mut self) {
        self.suggestions.clear();
    }
}

#[no_mangle]
pub extern "C" fn rel_group_suggest_add_suggestion(module: *mut RelGroupSuggest, suggestion: *const u8, len: usize) -> i32 {
    if module.is_null() || suggestion.is_null() || len == 0 {
        return -1;
    }

    let slice = unsafe { core::slice::from_raw_parts(suggestion, len) };
    let str_slice = core::str::from_utf8(slice).map_err(|_| -1).unwrap_or(-1);
    if str_slice == "-1" {
        return -1;
    }

    let suggestion_str = String::from(str_slice);
    unsafe { (*module).add_suggestion(suggestion_str) };
    0
}

#[no_mangle]
pub extern "C" fn rel_group_suggest_remove_suggestion(module: *mut RelGroupSuggest, index: usize) -> i32 {
    if module.is_null() || index >= unsafe { (*module).suggestions.len() } {
        return -1;
    }

    unsafe { (*module).remove_suggestion(index) };
    0
}

#[no_mangle]
pub extern "C" fn rel_group_suggest_get_suggestion(module: *const RelGroupSuggest, index: usize, buffer: *mut u8, len: usize) -> i32 {
    if module.is_null() || index >= unsafe { (*module).suggestions.len() } || buffer.is_null() || len == 0 {
        return -1;
    }

    let suggestion = unsafe { (*module).get_suggestion(index) }.unwrap();
    let suggestion_bytes = suggestion.as_bytes();

    if len < suggestion_bytes.len() + 1 {
        return -1;
    }

    unsafe {
        core::ptr::copy_nonoverlapping(suggestion_bytes.as_ptr(), buffer, suggestion_bytes.len());
        *buffer.offset(suggestion_bytes.len() as isize) = 0; // Null-terminate the string
    }
    0
}

#[no_mangle]
pub extern "C" fn rel_group_suggest_list_suggestions(module: *const RelGroupSuggest, buffer: *mut u8, len: usize) -> i32 {
    if module.is_null() || buffer.is_null() || len == 0 {
        return -1;
    }

    let suggestions = unsafe { (*module).list_suggestions() };
    let mut total_len = 0;

    for suggestion in suggestions {
        total_len += suggestion.len() + 1; // +1 for null-terminator
    }

    if len < total_len {
        return -1;
    }

    let mut current_offset = 0;
    for suggestion in suggestions {
        unsafe {
            core::ptr::copy_nonoverlapping(suggestion.as_ptr(), buffer.offset(current_offset as isize), suggestion.len());
            *buffer.offset((current_offset + suggestion.len()) as isize) = 0; // Null-terminate the string
        }
        current_offset += suggestion.len() + 1;
    }

    0
}

#[no_mangle]
pub extern "C" fn rel_group_suggest_clear_suggestions(module: *mut RelGroupSuggest) -> i32 {
    if module.is_null() {
        return -1;
    }

    unsafe { (*module).clear_suggestions() };
    0
}
