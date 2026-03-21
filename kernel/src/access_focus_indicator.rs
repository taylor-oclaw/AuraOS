extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AccessFocusIndicator {
    focus_list: Vec<String>,
    current_focus_index: usize,
}

impl AccessFocusIndicator {
    pub fn new() -> Self {
        AccessFocusIndicator {
            focus_list: Vec::new(),
            current_focus_index: 0,
        }
    }

    pub fn add_focus(&mut self, item: String) {
        self.focus_list.push(item);
    }

    pub fn remove_focus(&mut self, index: usize) -> Option<String> {
        if index < self.focus_list.len() {
            Some(self.focus_list.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_focus(&self) -> Option<&String> {
        if self.current_focus_index < self.focus_list.len() {
            Some(&self.focus_list[self.current_focus_index])
        } else {
            None
        }
    }

    pub fn set_next_focus(&mut self) {
        if !self.focus_list.is_empty() {
            self.current_focus_index = (self.current_focus_index + 1) % self.focus_list.len();
        }
    }

    pub fn set_previous_focus(&mut self) {
        if !self.focus_list.is_empty() {
            if self.current_focus_index == 0 {
                self.current_focus_index = self.focus_list.len() - 1;
            } else {
                self.current_focus_index -= 1;
            }
        }
    }
}
