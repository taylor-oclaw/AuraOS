extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TextBuffer {
    pub lines: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            lines: {
                let mut v = Vec::new();
                v.push(String::new());
                v
            },
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        if ch == '\n' {
            let rest = String::from(&self.lines[self.cursor_y][self.cursor_x..]);
            self.lines[self.cursor_y].truncate(self.cursor_x);
            self.cursor_y += 1;
            self.lines.insert(self.cursor_y, rest);
            self.cursor_x = 0;
        } else {
            self.lines[self.cursor_y].insert(self.cursor_x, ch);
            self.cursor_x += 1;
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
            self.lines[self.cursor_y].remove(self.cursor_x);
        } else if self.cursor_y > 0 {
            let line = self.lines.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = self.lines[self.cursor_y].len();
            self.lines[self.cursor_y].push_str(&line);
        }
    }

    pub fn move_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor_y + 1 < self.lines.len() {
            self.cursor_y += 1;
            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor_x < self.lines[self.cursor_y].len() {
            self.cursor_x += 1;
        }
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn total_chars(&self) -> usize {
        self.lines.iter().map(|l| l.len()).sum()
    }
}
