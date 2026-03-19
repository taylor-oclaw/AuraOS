extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FbConsole {
    pub cols: usize,
    pub rows: usize,
    pub cursor_col: usize,
    pub cursor_row: usize,
    pub buffer: Vec<Vec<char>>,
    pub fg_color: u32,
    pub bg_color: u32
}

impl FbConsole {
    pub fn new(cols: usize, rows: usize) -> Self {
        let buffer = {
            let mut b = Vec::new();
            for _ in 0..rows {
                let mut row = Vec::new();
                for _ in 0..cols {
                    row.push(32 as char);
                }
                b.push(row);
            }
            b
        };
        Self {
            cols,
            rows,
            cursor_col: 0,
            cursor_row: 0,
            buffer,
            fg_color: 0xD8DEE9,
            bg_color: 0x2E3440
        }
    }

    pub fn write_char(&mut self, ch: char) {
        if ch == 10 as char {
            self.cursor_col = 0;
            self.cursor_row += 1;
            if self.cursor_row >= self.rows {
                self.scroll_up();
                self.cursor_row = self.rows - 1;
            }
            return;
        }
        self.buffer[self.cursor_row][self.cursor_col] = ch;
        self.cursor_col += 1;
        if self.cursor_col >= self.cols {
            self.cursor_col = 0;
            self.cursor_row += 1;
            if self.cursor_row >= self.rows {
                self.scroll_up();
                self.cursor_row = self.rows - 1;
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for ch in s.chars() {
            self.write_char(ch);
        }
    }

    pub fn scroll_up(&mut self) {
        self.buffer.remove(0);
        let mut new_row = Vec::new();
        for _ in 0..self.cols {
            new_row.push(32 as char);
        }
        self.buffer.push(new_row);
    }

    pub fn clear(&mut self) {
        for row in self.buffer.iter_mut() {
            for ch in row.iter_mut() {
                *ch = 32 as char;
            }
        }
        self.cursor_col = 0;
        self.cursor_row = 0;
    }
}
