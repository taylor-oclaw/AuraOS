extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AppWebScraper {
    url: String,
    content: Vec<u8>,
    status_code: u16,
    headers: Vec<(String, String)>,
}

impl AppWebScraper {
    pub fn new(url: &str) -> Self {
        AppWebScraper {
            url: String::from(url),
            content: Vec::new(),
            status_code: 0,
            headers: Vec::new(),
        }
    }

    pub fn fetch(&mut self) {
        // Simulate fetching the URL
        self.status_code = 200;
        self.content = b"Sample HTML content".to_vec();
        self.headers.push((String::from("Content-Type"), String::from("text/html")));
    }

    pub fn get_status_code(&self) -> u16 {
        self.status_code
    }

    pub fn get_content(&self) -> &[u8] {
        &self.content
    }

    pub fn get_headers(&self) -> &[(String, String)] {
        &self.headers
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = String::from(url);
    }
}
