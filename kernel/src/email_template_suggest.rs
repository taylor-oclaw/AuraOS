extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailTemplateSuggest {
    templates: Vec<String>,
}

impl EmailTemplateSuggest {
    pub fn new() -> Self {
        EmailTemplateSuggest {
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

    pub fn suggest_template(&self, keyword: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        for template in &self.templates {
            if template.contains(keyword) {
                suggestions.push(template.clone());
            }
        }
        suggestions
    }

    pub fn list_templates(&self) -> Vec<&String> {
        self.templates.iter().collect()
    }
}
