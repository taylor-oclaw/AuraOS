extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NativeContextWindowManage {
    windows: Vec<Window>,
}

impl NativeContextWindowManage {
    pub fn new() -> Self {
        NativeContextWindowManage {
            windows: Vec::new(),
        }
    }

    pub fn add_window(&mut self, title: String, content: String) {
        let window = Window { title, content };
        self.windows.push(window);
    }

    pub fn remove_window_by_title(&mut self, title: &str) -> bool {
        if let Some(index) = self.windows.iter().position(|w| w.title == title) {
            self.windows.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_window_content_by_title(&self, title: &str) -> Option<&String> {
        self.windows.iter().find_map(|w| {
            if w.title == title {
                Some(&w.content)
            } else {
                None
            }
        })
    }

    pub fn list_windows(&self) -> Vec<&Window> {
        self.windows.iter().collect()
    }

    pub fn update_window_content(&mut self, title: &str, new_content: String) -> bool {
        if let Some(window) = self.windows.iter_mut().find(|w| w.title == title) {
            window.content = new_content;
            true
        } else {
            false
        }
    }
}

struct Window {
    title: String,
    content: String,
}