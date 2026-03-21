extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarBufferTime {
    buffer: Vec<String>,
}

impl CalendarBufferTime {
    pub fn new() -> Self {
        CalendarBufferTime {
            buffer: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: String) {
        self.buffer.push(event);
    }

    pub fn remove_event(&mut self, index: usize) -> Option<String> {
        if index < self.buffer.len() {
            Some(self.buffer.remove(index))
        } else {
            None
        }
    }

    pub fn get_event(&self, index: usize) -> Option<&String> {
        self.buffer.get(index)
    }

    pub fn list_events(&self) -> &[String] {
        &self.buffer
    }

    pub fn clear_events(&mut self) {
        self.buffer.clear();
    }
}
