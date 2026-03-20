extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::str::FromStr;

pub struct CrucibleRegex {
    pattern: String,
}

impl CrucibleRegex {
    pub fn new(pattern: &str) -> Self {
        CrucibleRegex {
            pattern: String::from_str(pattern).unwrap(),
        }
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.pattern == text
    }

    pub fn find(&self, text: &str) -> Option<usize> {
        if text.contains(&self.pattern) {
            Some(text.find(&self.pattern).unwrap())
        } else {
            None
        }
    }

    pub fn replace(&self, text: &str, replacement: &str) -> String {
        let mut result = String::new();
        let mut last_end = 0;
        for match_ in self.find_iter(text) {
            result.push_str(&text[last_end..match_.start()]);
            result.push_str(replacement);
            last_end = match_.end();
        }
        result.push_str(&text[last_end..]);
        result
    }

    pub fn find_iter<'a>(&self, text: &'a str) -> FindIter<'a> {
        FindIter {
            regex: self,
            text,
            position: 0,
        }
    }

    pub fn split(&self, text: &str) -> Vec<&str> {
        let mut result = Vec::new();
        let mut last_end = 0;
        for match_ in self.find_iter(text) {
            if match_.start() > last_end {
                result.push(&text[last_end..match_.start()]);
            }
            last_end = match_.end();
        }
        if last_end < text.len() {
            result.push(&text[last_end..]);
        }
        result
    }
}

pub struct FindIter<'a> {
    regex: &'a CrucibleRegex,
    text: &'a str,
    position: usize,
}

impl<'a> Iterator for FindIter<'a> {
    type Item = core::ops::Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(position) = self.text[self.position..].find(&self.regex.pattern) {
            let start = position + self.position;
            let end = start + self.regex.pattern.len();
            self.position = end;
            Some(start..end)
        } else {
            None
        }
    }
}
