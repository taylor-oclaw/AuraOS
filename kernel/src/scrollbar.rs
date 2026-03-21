extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct Scrollbar {
    position: usize,
    max_position: usize,
    page_size: usize,
}

impl Scrollbar {
    pub fn new(max_position: usize, page_size: usize) -> Self {
        Scrollbar {
            position: 0,
            max_position,
            page_size,
        }
    }

    pub fn get_position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, position: usize) {
        if position <= self.max_position {
            self.position = position;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.position < self.max_position {
            self.position += 1;
        }
    }

    pub fn page_up(&mut self) {
        if self.position >= self.page_size {
            self.position -= self.page_size;
        } else {
            self.position = 0;
        }
    }

    pub fn page_down(&mut self) {
        let new_position = self.position + self.page_size;
        if new_position > self.max_position {
            self.position = self.max_position;
        } else {
            self.position = new_position;
        }
    }
}
