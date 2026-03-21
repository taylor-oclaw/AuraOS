extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct SubtitleManager {
    subtitles: Vec<String>,
}

impl SubtitleManager {
    pub fn new() -> Self {
        SubtitleManager {
            subtitles: Vec::new(),
        }
    }

    pub fn add_subtitle(&mut self, subtitle: String) {
        self.subtitles.push(subtitle);
    }

    pub fn remove_subtitle(&mut self, index: usize) -> Option<String> {
        if index < self.subtitles.len() {
            Some(self.subtitles.remove(index))
        } else {
            None
        }
    }

    pub fn get_subtitle(&self, index: usize) -> Option<&String> {
        self.subtitles.get(index)
    }

    pub fn list_subtitles(&self) -> &[String] {
        &self.subtitles
    }

    pub fn clear_subtitles(&mut self) {
        self.subtitles.clear();
    }
}
