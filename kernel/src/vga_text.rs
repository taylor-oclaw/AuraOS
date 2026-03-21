extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct VgaText {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
    cursor_x: usize,
    cursor_y: usize,
}

impl VgaText {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer_size = width * height;
        let buffer = vec![0; buffer_size];
        VgaText {
            buffer,
            width,
            height,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn clear(&mut self) {
        for byte in self.buffer.iter_mut() {
            *byte = 0;
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    pub fn write_char(&mut self, c: char) {
        if c == '\n' {
            self.newline();
        } else {
            let byte = c as u8;
            let index = self.cursor_y * self.width + self.cursor_x;
            if index < self.buffer.len() {
                self.buffer[index] = byte;
            }
            self.move_cursor(1);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn newline(&mut self) {
        self.cursor_x = 0;
        self.move_cursor(self.width);
    }

    fn move_cursor(&mut self, offset: usize) {
        self.cursor_x += offset;
        if self.cursor_x >= self.width {
            let lines = self.cursor_x / self.width;
            self.cursor_y += lines;
            self.cursor_x %= self.width;
        }
        if self.cursor_y >= self.height {
            self.scroll_up();
            self.cursor_y = self.height - 1;
        }
    }

    fn scroll_up(&mut self) {
        let line_size = self.width;
        for i in 0..(self.buffer.len() - line_size) {
            self.buffer[i] = self.buffer[i + line_size];
        }
        for i in (self.buffer.len() - line_size)..self.buffer.len() {
            self.buffer[i] = 0;
        }
    }
}
