extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingTemplateMgr {
    templates: Vec<String>,
}

impl MeetingTemplateMgr {
    pub fn new() -> Self {
        MeetingTemplateMgr {
            templates: Vec::new(),
        }
    }

    pub fn add_template(&mut self, template: String) {
        self.templates.push(template);
    }

    pub fn remove_template(&mut self, index: usize) -> Option<String> {
        if index < self.templates.len() {
            Some(self.templates.remove(index))
        } else {
            None
        }
    }

    pub fn get_template(&self, index: usize) -> Option<&String> {
        self.templates.get(index)
    }

    pub fn list_templates(&self) -> &[String] {
        &self.templates
    }

    pub fn clear_templates(&mut self) {
        self.templates.clear();
    }
}
