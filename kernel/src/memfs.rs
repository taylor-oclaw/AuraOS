//! In-memory filesystem
//! Simple tree-based filesystem that lives entirely in RAM.
//! No disk persistence — contents lost on reboot.
//! Used for temp files, config, and AI model staging.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FileSystem {
    files: Vec<FileEntry>,
}

#[derive(Clone)]
pub struct FileEntry {
    pub path: String,
    pub content: Vec<u8>,
    pub is_dir: bool,
}

impl FileSystem {
    pub fn new() -> Self {
        let mut fs = FileSystem { files: Vec::new() };
        // Create root
        fs.files.push(FileEntry {
            path: String::from("/"),
            content: Vec::new(),
            is_dir: true,
        });
        fs
    }

    pub fn mkdir(&mut self, path: &str) -> bool {
        if self.exists(path) { return false; }
        // Create parent dirs if needed
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut current = String::from("");
        for part in &parts {
            current.push('/');
            current.push_str(part);
            if !self.exists(&current) {
                self.files.push(FileEntry {
                    path: current.clone(),
                    content: Vec::new(),
                    is_dir: true,
                });
            }
        }
        true
    }

    pub fn create_file(&mut self, path: &str, content: &[u8]) -> bool {
        // Ensure parent directory exists
        if let Some(parent) = parent_path(path) {
            if !self.exists(&parent) {
                self.mkdir(&parent);
            }
        }
        // Remove existing file with same path
        self.files.retain(|f| f.path != path);
        self.files.push(FileEntry {
            path: String::from(path),
            content: Vec::from(content),
            is_dir: false,
        });
        true
    }

    pub fn read_file(&self, path: &str) -> Option<&[u8]> {
        self.files.iter()
            .find(|f| f.path == path && !f.is_dir)
            .map(|f| f.content.as_slice())
    }

    pub fn list_dir(&self, path: &str) -> Vec<String> {
        let prefix = if path.ends_with('/') {
            String::from(path)
        } else {
            let mut s = String::from(path);
            s.push('/');
            s
        };
        
        self.files.iter()
            .filter(|f| {
                f.path.starts_with(prefix.as_str()) && f.path != path
            })
            .filter(|f| {
                // Only direct children (no nested)
                let remainder = &f.path[prefix.len()..];
                !remainder.contains('/')
            })
            .map(|f| {
                let name = f.path.split('/').last().unwrap_or("");
                String::from(name)
            })
            .collect()
    }

    pub fn exists(&self, path: &str) -> bool {
        self.files.iter().any(|f| f.path == path)
    }

    pub fn delete(&mut self, path: &str) -> bool {
        let prefix = {
            let mut s = String::from(path);
            s.push('/');
            s
        };
        // Remove the file/dir and all children
        let before = self.files.len();
        self.files.retain(|f| f.path != path && !f.path.starts_with(prefix.as_str()));
        self.files.len() < before
    }

    pub fn file_count(&self) -> usize {
        self.files.iter().filter(|f| !f.is_dir).count()
    }

    pub fn dir_count(&self) -> usize {
        self.files.iter().filter(|f| f.is_dir).count()
    }
}

fn parent_path(path: &str) -> Option<String> {
    let trimmed = path.trim_end_matches('/');
    if let Some(pos) = trimmed.rfind('/') {
        if pos == 0 {
            Some(String::from("/"))
        } else {
            Some(String::from(&trimmed[..pos]))
        }
    } else {
        None
    }
}

/// Global filesystem instance
static FS: spin::Lazy<spin::Mutex<FileSystem>> = spin::Lazy::new(|| {
    spin::Mutex::new(FileSystem::new())
};

pub fn with_fs<F: FnOnce(&mut FileSystem) -> R, R>(f: F) -> R {
    f(&mut FS.lock())
)}
