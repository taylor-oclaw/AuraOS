extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub struct ProgressBar {
    total: usize,
    current: usize,
    width: usize,
}

impl ProgressBar {
    pub fn new(total: usize, width: usize) -> Self {
        ProgressBar {
            total,
            current: 0,
            width,
        }
    }

    pub fn update(&mut self, increment: usize) {
        if self.current + increment > self.total {
            self.current = self.total;
        } else {
            self.current += increment;
        }
    }

    pub fn set_current(&mut self, current: usize) {
        if current > self.total {
            self.current = self.total;
        } else {
            self.current = current;
        }
    }

    pub fn get_progress(&self) -> f32 {
        (self.current as f32 / self.total as f32) * 100.0
    }

    pub fn render(&self) -> String {
        let filled_width = (self.get_progress() / 100.0 * self.width as f32).round() as usize;
        let empty_width = self.width - filled_width;

        let mut bar = String::from("[");
        for _ in 0..filled_width {
            bar.push('#');
        }
        for _ in 0..empty_width {
            bar.push(' ');
        }
        bar.push_str("]");

        bar
    }
}
