extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

pub struct AuraWebBrowser {
    history: Vec<String>,
    current_page: String,
    bookmarks: Vec<String>,
    is_incognito: bool,
    cache_size: usize,
}

impl AuraWebBrowser {
    pub fn new() -> Self {
        AuraWebBrowser {
            history: Vec::new(),
            current_page: String::from("about:blank"),
            bookmarks: Vec::new(),
            is_incognito: false,
            cache_size: 0,
        }
    }

    pub fn navigate(&mut self, url: &str) {
        if !self.is_incognito {
            self.history.push(self.current_page.clone());
        }
        self.current_page = String::from(url);
        self.cache_size += 1;
    }

    pub fn go_back(&mut self) -> Option<&String> {
        if let Some(page) = self.history.pop() {
            self.current_page = page;
            Some(&self.current_page)
        } else {
            None
        }
    }

    pub fn add_bookmark(&mut self, url: &str) {
        self.bookmarks.push(String::from(url));
    }

    pub fn remove_bookmark(&mut self, url: &str) -> bool {
        if let Some(index) = self.bookmarks.iter().position(|b| b == url) {
            self.bookmarks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_bookmarks(&self) -> Vec<&String> {
        self.bookmarks.iter().collect()
    }
}
