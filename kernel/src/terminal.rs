extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub const TERM_COLS: usize = 80;
pub const TERM_ROWS: usize = 25;

pub struct TermCell {
    pub ch: char,
    pub fg: u32,
    pub bg: u32,
}

pub struct Terminal {
    pub cells: Vec<Vec<TermCell>>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub cols: usize,
    pub rows: usize,
    pub input_buffer: String,
    pub prompt: String,
    pub scroll_offset: usize,
    pub history: Vec<String>,
}

impl Terminal {
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut cells = Vec::new();
        for _ in 0..rows {
            let mut row = Vec::new();
            for _ in 0..cols {
                row.push(TermCell {
                    ch: ' ',
                    fg: 0xD8DEE9,
                    bg: 0x2E3440,
                });
            }
            cells.push(row);
        }
        Self {
            cells,
            cursor_x: 0,
            cursor_y: 0,
            cols,
            rows,
            input_buffer: String::new(),
            prompt: String::from("aura> "),
            scroll_offset: 0,
            history: Vec::new(),
        }
    }

    pub fn write_char(&mut self, ch: char, fg: u32) {
        if ch == '\n' {
            self.cursor_x = 0;
            self.cursor_y += 1;
            if self.cursor_y >= self.rows {
                self.scroll_up();
                self.cursor_y = self.rows - 1;
            }
            return;
        }
        if self.cursor_x < self.cols && self.cursor_y < self.rows {
            self.cells[self.cursor_y][self.cursor_x] = TermCell {
                ch,
                fg,
                bg: 0x2E3440,
            };
            self.cursor_x += 1;
            if self.cursor_x >= self.cols {
                self.cursor_x = 0;
                self.cursor_y += 1;
                if self.cursor_y >= self.rows {
                    self.scroll_up();
                    self.cursor_y = self.rows - 1;
                }
            }
        }
    }

    pub fn write_str(&mut self, s: &str, fg: u32) {
        for ch in s.chars() {
            self.write_char(ch, fg);
        }
    }

    pub fn scroll_up(&mut self) {
        self.cells.remove(0);
        let mut new_row = Vec::new();
        for _ in 0..self.cols {
            new_row.push(TermCell {
                ch: ' ',
                fg: 0xD8DEE9,
                bg: 0x2E3440,
            });
        }
        self.cells.push(new_row);
    }

    pub fn handle_key(&mut self, ch: char) {
        if ch == '\n' {
            let cmd = self.input_buffer.clone();
            self.history.push(cmd.clone());
            self.write_char('\n', 0);
            self.input_buffer.clear();
            self.show_prompt();
        } else if ch == '\x08' { // Backspace
            if !self.input_buffer.is_empty() {
                self.input_buffer.pop();
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                    self.cells[self.cursor_y][self.cursor_x] = TermCell {
                        ch: ' ',
                        fg: 0xD8DEE9,
                        bg: 0x2E3440,
                    };
                }
            }
        } else {
            self.input_buffer.push(ch);
            self.write_char(ch, 0xD8DEE9);
        }
    }

    pub fn show_prompt(&mut self) {
        self.write_str(&self.prompt.clone(), 0x88C0D0);
    }

    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                cell.ch = ' ';
            }
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
    }
}
