extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AsfManifestParser {
    manifest_data: Vec<u8>,
}

impl AsfManifestParser {
    pub fn new(manifest_data: Vec<u8>) -> Self {
        AsfManifestParser { manifest_data }
    }

    pub fn get_manifest_version(&self) -> Option<String> {
        self.find_key_value("version")
    }

    pub fn get_author(&self) -> Option<String> {
        self.find_key_value("author")
    }

    pub fn get_description(&self) -> Option<String> {
        self.find_key_value("description")
    }

    pub fn get_license(&self) -> Option<String> {
        self.find_key_value("license")
    }

    pub fn get_dependencies(&self) -> Option<Vec<String>> {
        if let Some(value) = self.find_key_value("dependencies") {
            value.split(',').map(|s| s.trim().to_string()).collect()
        } else {
            None
        }
    }

    fn find_key_value(&self, key: &str) -> Option<String> {
        let mut start_index = 0;
        while let Some(index) = self.manifest_data[start_index..].find(key.as_bytes()) {
            let index = index + start_index;
            if index > 0 && self.manifest_data[index - 1] == b'\n' || index == 0 {
                let line_start = index + key.len();
                if line_start < self.manifest_data.len() && self.manifest_data[line_start] == b':' {
                    let value_start = line_start + 1;
                    if value_start < self.manifest_data.len() {
                        let mut end_index = value_start;
                        while end_index < self.manifest_data.len() && self.manifest_data[end_index] != b'\n' {
                            end_index += 1;
                        }
                        return Some(String::from_utf8_lossy(&self.manifest_data[value_start..end_index]).trim().to_string());
                    }
                }
            }
            start_index = index + key.len();
        }
        None
    }
}
