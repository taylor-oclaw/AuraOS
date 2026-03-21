extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_apology_draft_init() {
    // Initialization code for the module
}

pub extern "C" fn rel_apology_draft_exit() {
    // Cleanup code for the module
}

pub struct ApologyDraft {
    title: String,
    content: String,
    author: String,
    recipients: Vec<String>,
    version: u32,
}

impl ApologyDraft {
    pub fn new(title: &str, content: &str, author: &str) -> Self {
        ApologyDraft {
            title: String::from(title),
            content: String::from(content),
            author: String::from(author),
            recipients: Vec::new(),
            version: 1,
        }
    }

    pub fn add_recipient(&mut self, recipient: &str) {
        self.recipients.push(String::from(recipient));
    }

    pub fn remove_recipient(&mut self, recipient: &str) -> bool {
        if let Some(index) = self.recipients.iter().position(|r| r == recipient) {
            self.recipients.remove(index);
            true
        } else {
            false
        }
    }

    pub fn update_content(&mut self, new_content: &str) {
        self.content = String::from(new_content);
        self.version += 1;
    }

    pub fn get_apology_details(&self) -> (String, String, String, Vec<String>, u32) {
        (
            self.title.clone(),
            self.content.clone(),
            self.author.clone(),
            self.recipients.clone(),
            self.version,
        
    }
}
