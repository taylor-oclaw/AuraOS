extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub path: String,
}

pub struct FileManager {
    pub current_path: String,
    pub entries: Vec<FileEntry>,
    pub selected: Option<usize>,
    pub history: Vec<String>,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            current_path: String::from("/"),
            entries: Vec::new(),
            selected: None,
            history: Vec::new(),
        }
    }

    pub fn navigate(&mut self, path: &str) {
        self.history.push(self.current_path.clone());
        self.current_path = String::from(path);
        self.selected = None;
    }

    pub fn go_back(&mut self) {
        if let Some(prev) = self.history.pop() {
            self.current_path = prev;
            self.selected = None;
        }
    }

    pub fn go_up(&mut self) {
        if self.current_path != "/" {
            let mut parts: Vec<&str> = self.current_path.split(47 as char).filter(|s| !s.is_empty()).collect();
            parts.pop();
            self.history.push(self.current_path.clone());
            if parts.is_empty() {
                self.current_path = String::from("/");
            } else {
                let mut p = String::from("/");
                for part in parts {
                    p.push_str(part);
                    p.push(47 as char);
                }
                self.current_path = p;
            }
        }
    }

    pub fn select_next(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        self.selected = Some(self.selected.map(|s| (s + 1) % self.entries.len()).unwrap_or(0));
    }

    pub fn select_prev(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        self.selected = Some(self.selected.map(|s| if s == 0 { self.entries.len() - 1 } else { s - 1 }).unwrap_or(0));
    }

    pub fn selected_entry(&self) -> Option<&FileEntry> {
        self.selected.and_then(|i| self.entries.get(i))
    }

    pub fn add_entry(&mut self, name: &str, is_dir: bool, size: u64) {
        let path = if self.current_path.ends_with(47 as char) {
            alloc::String::from("error")
        } else {
            alloc::String::from("error")
        };
        self.entries.push(FileEntry {
            name: String::from(name),
            is_dir,
            size,
            path,
        });
    }
}
