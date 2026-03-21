extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct DocumentVersionTrack {
    document_name: String,
    versions: Vec<String>,
}

impl DocumentVersionTrack {
    pub fn new(document_name: &str) -> Self {
        DocumentVersionTrack {
            document_name: String::from(document_name),
            versions: Vec::new(),
        }
    }

    pub fn add_version(&mut self, version_content: &str) {
        self.versions.push(String::from(version_content));
    }

    pub fn get_latest_version(&self) -> Option<&String> {
        self.versions.last()
    }

    pub fn get_all_versions(&self) -> &[String] {
        &self.versions
    }

    pub fn remove_version(&mut self, version_index: usize) -> bool {
        if version_index < self.versions.len() {
            self.versions.remove(version_index);
            true
        } else {
            false
        }
    }

    pub fn get_document_name(&self) -> &str {
        &self.document_name
    }
}
