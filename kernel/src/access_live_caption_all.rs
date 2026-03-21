extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct LiveCaption {
    captions: Vec<String>,
    current_index: usize,
}

impl LiveCaption {
    pub fn new() -> Self {
        LiveCaption {
            captions: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_caption(&mut self, caption: String) {
        self.captions.push(caption);
    }

    pub fn get_current_caption(&self) -> Option<&String> {
        if self.current_index < self.captions.len() {
            Some(&self.captions[self.current_index])
        } else {
            None
        }
    }

    pub fn next_caption(&mut self) -> Option<&String> {
        if self.current_index + 1 < self.captions.len() {
            self.current_index += 1;
            Some(&self.captions[self.current_index])
        } else {
            None
        }
    }

    pub fn previous_caption(&mut self) -> Option<&String> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(&self.captions[self.current_index])
        } else {
            None
        }
    }

    pub fn remove_current_caption(&mut self) -> Option<String> {
        if let Some(caption) = self.captions.get(self.current_index).cloned() {
            self.captions.remove(self.current_index);
            if self.current_index >= self.captions.len() && !self.captions.is_empty() {
                self.current_index -= 1;
            }
            Some(caption)
        } else {
            None
        }
    }
}
