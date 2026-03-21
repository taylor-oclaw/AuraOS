extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct BizCrmNoteAttach {
    note_id: u32,
    attachments: Vec<String>,
}

impl BizCrmNoteAttach {
    pub fn new(note_id: u32) -> Self {
        BizCrmNoteAttach {
            note_id,
            attachments: Vec::new(),
        }
    }

    pub fn add_attachment(&mut self, attachment_name: &str) {
        let name = String::from(attachment_name);
        self.attachments.push(name);
    }

    pub fn remove_attachment(&mut self, attachment_name: &str) -> bool {
        if let Some(index) = self.attachments.iter().position(|x| x == attachment_name) {
            self.attachments.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_attachments(&self) -> &[String] {
        &self.attachments
    }

    pub fn has_attachment(&self, attachment_name: &str) -> bool {
        self.attachments.contains(&String::from(attachment_name))
    }

    pub fn clear_attachments(&mut self) {
        self.attachments.clear();
    }
}
