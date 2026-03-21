extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn tone_draft_preview_perception_init() {
    // Initialization logic for the module
}

pub extern "C" fn tone_draft_preview_perception_exit() {
    // Cleanup logic for the module
}

pub struct ToneDraftPreviewPerception {
    drafts: Vec<String>,
    current_index: usize,
}

impl ToneDraftPreviewPerception {
    pub fn new() -> Self {
        ToneDraftPreviewPerception {
            drafts: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_draft(&mut self, draft: String) {
        self.drafts.push(draft);
    }

    pub fn remove_draft(&mut self, index: usize) -> Option<String> {
        if index < self.drafts.len() {
            Some(self.drafts.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_draft(&self) -> Option<&String> {
        if self.current_index < self.drafts.len() {
            Some(&self.drafts[self.current_index])
        } else {
            None
        }
    }

    pub fn next_draft(&mut self) {
        if self.current_index + 1 < self.drafts.len() {
            self.current_index += 1;
        }
    }

    pub fn previous_draft(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }
}
