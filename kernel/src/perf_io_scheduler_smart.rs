extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct SmartIOScheduler {
    queue: Vec<String>,
    current_index: usize,
}

impl SmartIOScheduler {
    pub fn new() -> Self {
        SmartIOScheduler {
            queue: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_request(&mut self, request: String) {
        self.queue.push(request);
    }

    pub fn remove_request(&mut self, index: usize) -> Option<String> {
        if index < self.queue.len() {
            Some(self.queue.remove(index))
        } else {
            None
        }
    }

    pub fn get_next_request(&self) -> Option<&String> {
        if self.current_index < self.queue.len() {
            Some(&self.queue[self.current_index])
        } else {
            None
        }
    }

    pub fn complete_current_request(&mut self) {
        if self.current_index < self.queue.len() {
            self.current_index += 1;
        }
    }

    pub fn reset_scheduler(&mut self) {
        self.current_index = 0;
    }
}
