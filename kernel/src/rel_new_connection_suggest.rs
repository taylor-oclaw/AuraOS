extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_new_connection_suggest_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_new_connection_suggest_exit() {
    // Cleanup logic for the module
}

pub struct ConnectionSuggester {
    suggestions: Vec<String>,
}

impl ConnectionSuggester {
    pub fn new() -> Self {
        ConnectionSuggester {
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

pub extern "C" fn rel_new_connection_suggest_add_suggestion(suggester: *mut ConnectionSuggester, suggestion: *const u8, len: usize) -> i32 {
    if suggester.is_null() || suggestion.is_null() || len == 0 {
        return -1;
    }

    let suggester = unsafe { &mut *suggester };
    let suggestion_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(suggestion, len)) };
    suggester.add_suggestion(String::from(suggestion_str));
    0
}

pub extern "C" fn rel_new_connection_suggest_remove_suggestion(suggester: *mut ConnectionSuggester, index: usize) -> i32 {
    if suggester.is_null() {
        return -1;
    }

    let suggester = unsafe { &mut *suggester };
    match suggester.remove_suggestion(index) {
        Some(_) => 0,
        None => -1,
    }
}

pub extern "C" fn rel_new_connection_suggest_get_suggestion(suggester: *const ConnectionSuggester, index: usize, buffer: *mut u8, len: usize) -> i32 {
    if suggester.is_null() || buffer.is_null() || len == 0 {
        return -1;
    }

    let suggester = unsafe { &*suggester };
    match suggester.get_suggestion(index) {
        Some(suggestion) => {
            let suggestion_bytes = suggestion.as_bytes();
            if suggestion_bytes.len() > len {
                return -2; // Buffer too small
            }
            unsafe {
                core::ptr::copy_nonoverlapping(suggestion_bytes.as_ptr(), buffer, suggestion_bytes.len());
            }
            0
        },
        None => -1,
    }
}

pub extern "C" fn rel_new_connection_suggest_list_suggestions(suggester: *const ConnectionSuggester, buffer: *mut u8, len: usize) -> i32 {
    if suggester.is_null() || buffer.is_null() || len == 0 {
        return -1;
    }

    let suggester = unsafe { &*suggester };
    let suggestions_str = suggester.list_suggestions().join(",");
    let suggestion_bytes = suggestions_str.as_bytes();
    if suggestion_bytes.len() > len {
        return -2; // Buffer too small
    }
    unsafe {
        core::ptr::copy_nonoverlapping(suggestion_bytes.as_ptr(), buffer, suggestion_bytes.len());
    }
    0
}

pub extern "C" fn rel_new_connection_suggest_clear_suggestions(suggester: *mut ConnectionSuggester) -> i32 {
    if suggester.is_null() {
        return -1;
    }

    let suggester = unsafe { &mut *suggester };
    suggester.clear_suggestions();
    0
}
