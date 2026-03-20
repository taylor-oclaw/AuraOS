extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CursorManager {
    positions: Vec<(u32, u32)>,
    current_index: usize,
}

impl CursorManager {
    pub fn new() -> Self {
        CursorManager {
            positions: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_position(&mut self, x: u32, y: u32) {
        self.positions.push((x, y));
        if self.current_index == self.positions.len() - 1 {
            self.current_index += 1;
        }
    }

    pub fn remove_current_position(&mut self) {
        if !self.positions.is_empty() {
            self.positions.remove(self.current_index);
            if self.current_index >= self.positions.len() {
                self.current_index = self.positions.len().saturating_sub(1);
            }
        }
    }

    pub fn move_to_next(&mut self) {
        if self.current_index < self.positions.len() - 1 {
            self.current_index += 1;
        }
    }

    pub fn move_to_previous(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }

    pub fn get_current_position(&self) -> Option<(u32, u32)> {
        self.positions.get(self.current_index).cloned()
    }
}
