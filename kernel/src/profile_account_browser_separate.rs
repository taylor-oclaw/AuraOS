extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileAccountBrowser {
    username: String,
    email: String,
    age: u8,
    browsing_history: Vec<String>,
    bookmarks: Vec<String>,
}

impl ProfileAccountBrowser {
    pub fn new(username: &str, email: &str, age: u8) -> Self {
        ProfileAccountBrowser {
            username: String::from(username),
            email: String::from(email),
            age,
            browsing_history: Vec::new(),
            bookmarks: Vec::new(),
        }
    }

    pub fn add_to_browsing_history(&mut self, url: &str) {
        self.browsing_history.push(String::from(url));
    }

    pub fn get_browsing_history(&self) -> &Vec<String> {
        &self.browsing_history
    }

    pub fn add_bookmark(&mut self, bookmark: &str) {
        self.bookmarks.push(String::from(bookmark));
    }

    pub fn remove_bookmark(&mut self, bookmark: &str) {
        if let Some(index) = self.bookmarks.iter().position(|b| b == bookmark) {
            self.bookmarks.remove(index);
        }
    }

    pub fn get_bookmarks(&self) -> &Vec<String> {
        &self.bookmarks
    }
}
