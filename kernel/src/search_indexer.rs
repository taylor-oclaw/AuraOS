extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct IndexEntry {
    pub file_id: u64,
    pub path: String,
    pub content_hash: [u8; 16],
    pub tags: Vec<String>,
    pub last_indexed: u64,
    pub file_size: u64
}

pub struct SearchResult {
    pub file_id: u64,
    pub path: String,
    pub relevance: f32,
    pub snippet: Option<String>
}

pub struct SearchIndexer {
    pub entries: Vec<IndexEntry>,
    pub total_indexed: u64
}

impl SearchIndexer {
    pub fn new() -> Self {
        Self { 
            entries: Vec::new(), 
            total_indexed: 0 
        }
    }

    pub fn index_file(&mut self, file_id: u64, path: &str, tags: Vec<String>, size: u64) {
        if let Some(e) = self.entries.iter_mut().find(|e| e.file_id == file_id) {
            e.tags = tags;
            e.last_indexed = 0;
        } else {
            self.entries.push(IndexEntry { 
                file_id, 
                path: String::from(path), 
                content_hash: [0; 16], 
                tags, 
                last_indexed: 0, 
                file_size: size 
            };
            self.total_indexed += 1;
        }
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let q = query.to_lowercase();
        self.entries.iter().filter(|e| {
            e.path.to_lowercase().contains(&q) || 
            e.tags.iter().any(|t| t.to_lowercase().contains(&q))
        }).map(|e| SearchResult { 
            file_id: e.file_id, 
            path: e.path.clone(), 
            relevance: 1.0, 
            snippet: None 
        }).collect()
    }

    pub fn search_by_tag(&self, tag: &str) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.tags.iter().any(|t| t == tag)).collect()
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
)}
