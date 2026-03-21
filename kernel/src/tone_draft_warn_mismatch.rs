extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn tone_draft_warn_mismatch_init() {
    // Initialization logic for the module
}

pub extern "C" fn tone_draft_warn_mismatch_exit() {
    // Cleanup logic for the module
}

pub struct ToneDraftWarnMismatch {
    drafts: Vec<String>,
    warnings: Vec<String>,
}

impl ToneDraftWarnMismatch {
    pub fn new() -> Self {
        ToneDraftWarnMismatch {
            drafts: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_draft(&mut self, draft: String) {
        self.drafts.push(draft);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn get_drafs_count(&self) -> usize {
        self.drafts.len()
    }

    pub fn get_warnings_count(&self) -> usize {
        self.warnings.len()
    }

    pub fn clear_all(&mut self) {
        self.drafts.clear();
        self.warnings.clear();
    }
}
