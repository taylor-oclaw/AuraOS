extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct DocumentOCRSmart {
    text: String,
    words: Vec<String>,
    lines: Vec<Vec<String>>,
}

impl DocumentOCRSmart {
    pub fn new(text: &str) -> Self {
        let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
        let lines: Vec<Vec<String>> = text.lines()
            .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
            .collect();

        DocumentOCRSmart {
            text: text.to_string(),
            words,
            lines,
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn count_words(&self) -> usize {
        self.words.len()
    }

    pub fn count_lines(&self) -> usize {
        self.lines.len()
    }

    pub fn find_word(&self, word: &str) -> Option<usize> {
        self.words.iter().position(|w| w == word)
    }

    pub fn get_line(&self, line_index: usize) -> Option<&Vec<String>> {
        self.lines.get(line_index)
    }
}
