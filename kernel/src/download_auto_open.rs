extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DownloadAutoOpen {
    url: String,
    filename: String,
}

impl DownloadAutoOpen {
    pub fn new(url: &str) -> Self {
        let mut download = DownloadAutoOpen {
            url: String::from(url),
            filename: String::new(),
        };
        download.get_filename();
        download
    }

    pub fn get_url(&self) -> &str {
        self.url.as_str()
    }

    pub fn set_url(&mut self, new_url: &str) {
        self.url = String::from(new_url);
        self.get_filename();
    }

    pub fn get_filename(&self) -> &str {
        let url_parts: Vec<&str> = self.url.split('/').collect();
        if let Some(filename) = url_parts.last() {
            return filename;
        }
        "unknown"
    }

    pub fn set_filename(&mut self, new_filename: &str) {
        self.filename = String::from(new_filename);
    }

    pub fn get_path(&self) -> String {
        format!("{}/{}", self.get_url(), self.get_filename())
    }
}