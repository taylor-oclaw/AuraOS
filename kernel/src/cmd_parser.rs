extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ParsedCommand {
    cmd: String,
    args: Vec<String>,
}

pub fn parse_command(input: &str) -> ParsedCommand {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let cmd = String::from(parts[0]);
    let args: Vec<String> = parts.iter().skip(1).map(|s| String::from(*s)).collect();
    ParsedCommand { cmd, args }
}

pub struct ShellHistory {
    entries: Vec<String>,
    max_entries: usize,
    cursor: usize,
}

impl ShellHistory {
    fn new(max_entries: usize) -> Self {
        ShellHistory {
            entries: Vec::new(),
            max_entries,
            cursor: 0,
        }
    }

    fn add(&mut self, cmd: String) {
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(cmd);
        self.cursor = self.entries.len();
    }

    fn up(&mut self) -> Option<String> {
        if self.cursor > 0 {
            self.cursor -= 1;
            Some(self.entries[self.cursor].clone())
        } else {
            None
        }
    }

    fn down(&mut self) -> Option<String> {
        if self.cursor < self.entries.len() - 1 {
            self.cursor += 1;
            Some(self.entries[self.cursor].clone())
        } else {
            None
        }
    }

    fn search(&self, query: &str) -> Vec<String> {
        self.entries
            .iter()
            .filter(|entry| entry.contains(query))
            .cloned()
            .collect()
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.cursor = 0;
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}
