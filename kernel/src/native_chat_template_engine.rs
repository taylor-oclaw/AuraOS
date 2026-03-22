extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NativeChatTemplateEngine {
    templates: Vec<String>,
}

impl NativeChatTemplateEngine {
    pub fn new() -> Self {
        NativeChatTemplateEngine {
            templates: Vec::new(),
        }
    }

    pub fn add_template(&mut self, template: String) {
        self.templates.push(template);
    }

    pub fn get_template(&self, index: usize) -> Option<&String> {
        self.templates.get(index)
    }

    pub fn remove_template(&mut self, index: usize) -> Option<String> {
        if index < self.templates.len() {
            Some(self.templates.remove(index))
        } else {
            None
        }
    }

    pub fn list_templates(&self) -> Vec<&String> {
        self.templates.iter().collect()
    }

    pub fn replace_placeholder(&mut self, template_index: usize, placeholder: &str, replacement: &str) -> Result<(), &'static str> {
        if let Some(template) = self.templates.get_mut(template_index) {
            let new_template = template.replace(placeholder, replacement);
            *template = new_template;
            Ok(())
        } else {
            Err("Template index out of bounds")
        }
    }
}