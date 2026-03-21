extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AppWindowControl {
    windows: Vec<Window>,
}

impl AppWindowControl {
    pub fn new() -> Self {
        AppWindowControl {
            windows: Vec::new(),
        }
    }

    pub fn add_window(&mut self, title: String, width: u32, height: u32) {
        let window = Window {
            title,
            width,
            height,
            is_visible: true,
        };
        self.windows.push(window);
    }

    pub fn remove_window_by_title(&mut self, title: &str) {
        self.windows.retain(|w| w.title != title);
    }

    pub fn get_window_count(&self) -> usize {
        self.windows.len()
    }

    pub fn toggle_visibility(&mut self, title: &str) {
        if let Some(window) = self.windows.iter_mut().find(|w| w.title == title) {
            window.is_visible = !window.is_visible;
        }
    }

    pub fn get_window_titles(&self) -> Vec<String> {
        self.windows.iter().map(|w| w.title.clone()).collect()
    }
}

struct Window {
    title: String,
    width: u32,
    height: u32,
    is_visible: bool,
}
