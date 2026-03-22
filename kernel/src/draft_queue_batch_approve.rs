extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DraftQueueBatchApprove {
    drafts: Vec<String>,
}

impl DraftQueueBatchApprove {
    pub fn new() -> Self {
        DraftQueueBatchApprove { drafts: Vec::new() }
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

    pub fn get_draft_count(&self) -> usize {
        self.drafts.len()
    }

    pub fn approve_all(&mut self) -> Vec<String> {
        let approved = self.drafts.clone();
        self.drafts.clear();
        approved
    }

    pub fn list_drafts(&self) -> Vec<&String> {
        self.drafts.iter().collect()
    }
}