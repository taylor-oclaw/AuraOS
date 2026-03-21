extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Cleanup logic for the module
}

pub struct FocusGuard {
    focus_list: Vec<String>,
    current_focus: Option<usize>,
}

impl FocusGuard {
    pub fn new() -> Self {
        FocusGuard {
            focus_list: Vec::new(),
            current_focus: None,
        }
    }

    pub fn add_focus(&mut self, item: String) {
        self.focus_list.push(item);
        if self.current_focus.is_none() {
            self.current_focus = Some(0);
        }
    }

    pub fn remove_focus(&mut self, index: usize) -> Option<String> {
        if let Some(current_index) = self.current_focus {
            if current_index >= index {
                self.current_focus = Some(current_index - 1);
            }
        }
        self.focus_list.remove(index)
    }

    pub fn get_current_focus(&self) -> Option<&String> {
        self.current_focus.map(|index| &self.focus_list[index])
    }

    pub fn set_next_focus(&mut self) {
        if let Some(current_index) = self.current_focus {
            let next_index = (current_index + 1) % self.focus_list.len();
            self.current_focus = Some(next_index);
        } else if !self.focus_list.is_empty() {
            self.current_focus = Some(0);
        }
    }

    pub fn set_previous_focus(&mut self) {
        if let Some(current_index) = self.current_focus {
            let prev_index = if current_index == 0 {
                self.focus_list.len() - 1
            } else {
                current_index - 1
            };
            self.current_focus = Some(prev_index);
        }
    }
}
