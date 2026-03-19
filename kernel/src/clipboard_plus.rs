extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum ClipboardContent {
    Text(String),
    Image {
        width: u32,
        height: u32,
        data: Vec<u8>,
    },
    File(String),
    RichText {
        html: String,
        plain: String,
    }
}

pub struct ClipboardEntry {
    pub id: u64,
    pub content: ClipboardContent,
    pub source_app: Option<String>,
    pub timestamp: u64,
    pub pinned: bool
}

pub struct ClipboardManager {
    pub history: Vec<ClipboardEntry>,
    pub max_history: usize,
    pub next_id: u64
}

impl ClipboardManager {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            max_history: 100,
            next_id: 1
        }
    }

    pub fn copy(&mut self, content: ClipboardContent, source: Option<&str>) {
        let id = self.next_id;
        self.next_id += 1;
        self.history.insert(0, ClipboardEntry {
            id,
            content,
            source_app: source.map(String::from),
            timestamp: 0,
            pinned: false
        });
        while self.history.len() > self.max_history {
            let last = self.history.len() - 1;
            if !self.history[last].pinned {
                self.history.remove(last);
            } else {
                break;
            }
        }
    }

    pub fn paste(&self) -> Option<&ClipboardContent> {
        self.history.first().map(|e| &e.content)
    }

    pub fn paste_nth(&self, n: usize) -> Option<&ClipboardContent> {
        self.history.get(n).map(|e| &e.content)
    }

    pub fn pin(&mut self, id: u64) {
        if let Some(e) = self.history.iter_mut().find(|e| e.id == id) {
            e.pinned = true;
        }
    }

    pub fn search(&self, query: &str) -> Vec<&ClipboardEntry> {
        let q = query.to_lowercase();
        self.history.iter().filter(|e| match &e.content {
            ClipboardContent::Text(t) => t.to_lowercase().contains(&q),
            ClipboardContent::RichText { plain, .. } => plain.to_lowercase().contains(&q),
            _ => false
        }).collect()
    }

    pub fn clear(&mut self) {
        self.history.retain(|e| e.pinned);
    }
}
