extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TextLayout {
    lines: Vec<String>,
    max_width: usize,
}

impl TextLayout {
    pub fn new(max_width: usize) -> Self {
        TextLayout {
            lines: Vec::new(),
            max_width,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut current_line = String::new();

        for word in words {
            if current_line.len() + word.len() + 1 > self.max_width {
                self.lines.push(current_line);
                current_line = String::from(word);
            } else if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }

        if !current_line.is_empty() {
            self.lines.push(current_line);
        }
    }

    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }

    pub fn total_characters(&self) -> usize {
        self.lines.iter().map(|line| line.len()).sum()
    }
}
