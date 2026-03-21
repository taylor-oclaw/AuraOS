extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct NotifyPresentationDetectV2 {
    presentations: Vec<String>,
    detected_count: usize,
}

impl NotifyPresentationDetectV2 {
    pub fn new() -> Self {
        NotifyPresentationDetectV2 {
            presentations: Vec::new(),
            detected_count: 0,
        }
    }

    pub fn add_presentation(&mut self, presentation_name: &str) {
        let name = String::from(presentation_name);
        self.presentations.push(name);
    }

    pub fn remove_presentation(&mut self, presentation_name: &str) -> bool {
        if let Some(index) = self.presentations.iter().position(|p| p == presentation_name) {
            self.presentations.remove(index);
            true
        } else {
            false
        }
    }

    pub fn detect_presentation(&mut self, presentation_name: &str) -> bool {
        if self.presentations.contains(&String::from(presentation_name)) {
            self.detected_count += 1;
            true
        } else {
            false
        }
    }

    pub fn get_detected_count(&self) -> usize {
        self.detected_count
    }

    pub fn list_presentations(&self) -> Vec<String> {
        self.presentations.clone()
    }
}
