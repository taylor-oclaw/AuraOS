extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn predict_routine_suggest_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn predict_routine_suggest_exit() {
    // Cleanup logic for the module
}

pub struct PredictRoutineSuggest {
    suggestions: Vec<String>,
}

impl PredictRoutineSuggest {
    pub fn new() -> Self {
        PredictRoutineSuggest {
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
pub extern "C" fn predict_routine_suggest_add(suggestion: *const u8, len: usize) -> i32 {
    let suggestion_str = unsafe { core::slice::from_raw_parts(suggestion, len) };
    if let Ok(suggestion_string) = String::from_utf8(suggestion_str.to_vec()) {
        let mut module = PredictRoutineSuggest::new();
        module.add_suggestion(suggestion_string);
        0 // Success
    } else {
        -1 // Error
    }
}

#[no_mangle]
pub extern "C" fn predict_routine_suggest_remove(index: usize) -> i32 {
    let mut module = PredictRoutineSuggest::new();
    if module.remove_suggestion(index).is_some() {
        0 // Success
    } else {
        -1 // Error
    }
}

#[no_mangle]
pub extern "C" fn predict_routine_suggest_get(index: usize, buffer: *mut u8, len: usize) -> i32 {
    let module = PredictRoutineSuggest::new();
    if let Some(suggestion) = module.get_suggestion(index) {
        let suggestion_bytes = suggestion.as_bytes();
        if suggestion_bytes.len() <= len {
            unsafe { core::ptr::copy_nonoverlapping(suggestion_bytes.as_ptr(), buffer, suggestion_bytes.len()) };
            0 // Success
        } else {
            -1 // Buffer too small
        }
    } else {
        -1 // Index out of bounds
    }
}

#[no_mangle]
pub extern "C" fn predict_routine_suggest_list(buffer: *mut u8, len: usize) -> i32 {
    let module = PredictRoutineSuggest::new();
    let suggestions_str = module.list_suggestions().join(",");
    let suggestion_bytes = suggestions_str.as_bytes();
    if suggestion_bytes.len() <= len {
        unsafe { core::ptr::copy_nonoverlapping(suggestion_bytes.as_ptr(), buffer, suggestion_bytes.len()) };
        0 // Success
    } else {
        -1 // Buffer too small
    }
}

#[no_mangle]
pub extern "C" fn predict_routine_suggest_clear() -> i32 {
    let mut module = PredictRoutineSuggest::new();
    module.clear_suggestions();
    0 // Success
}
