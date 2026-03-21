extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingClipGenerator {
    clips: Vec<String>,
}

impl MeetingClipGenerator {
    pub fn new() -> Self {
        MeetingClipGenerator { clips: Vec::new() }
    }

    pub fn add_clip(&mut self, clip: String) {
        self.clips.push(clip);
    }

    pub fn remove_clip(&mut self, index: usize) -> Option<String> {
        if index < self.clips.len() {
            Some(self.clips.remove(index))
        } else {
            None
        }
    }

    pub fn get_clip(&self, index: usize) -> Option<&String> {
        self.clips.get(index)
    }

    pub fn list_clips(&self) -> &[String] {
        &self.clips
    }

    pub fn clear_clips(&mut self) {
        self.clips.clear();
    }
}
