extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct AsfManifestParser {
    manifest_data: Vec<u8>,
}

impl AsfManifestParser {
    pub fn new(data: Vec<u8>) -> Self {
        AsfManifestParser {
            manifest_data: data,
        }
    }

    pub fn get_manifest_version(&self) -> Option<String> {
        self.find_key_value("version")
    }

    pub fn get_author(&self) -> Option<String> {
        self.find_key_value("author")
    }

    pub fn get_license(&self) -> Option<String> {
        self.find_key_value("license")
    }

    pub fn get_description(&self) -> Option<String> {
        self.find_key_value("description")
    }

    pub fn get_dependencies(&self) -> Option<Vec<String>> {
        if let Some(value) = self.find_key_value("dependencies") {
            value.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>().into()
        } else {
            None
        }
    }

    fn find_key_value(&self, key: &str) -> Option<String> {
        let mut lines = self.manifest_data.split(|&b| b == b'\n');
        while let Some(line) = lines.next() {
            if let Some((k, v)) = line.split_once(b'=') {
                if k.trim() == key.as_bytes() {
                    return String::from_utf8(v.trim().to_vec()).ok();
                }
            }
        }
        None
    }
}