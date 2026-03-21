extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangTranslateContextAware {
    context: Vec<String>,
}

impl LangTranslateContextAware {
    pub fn new() -> Self {
        LangTranslateContextAware {
            context: Vec::new(),
        }
    }

    pub fn add_context(&mut self, context: String) {
        self.context.push(context);
    }

    pub fn remove_context(&mut self, index: usize) -> Option<String> {
        if index < self.context.len() {
            Some(self.context.remove(index))
        } else {
            None
        }
    }

    pub fn get_context(&self, index: usize) -> Option<&String> {
        self.context.get(index)
    }

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn translate(&self, text: &str) -> String {
        // Dummy translation logic for demonstration purposes
        let mut translated = String::from(text);
        if self.context.contains(&String::from("en_to_fr")) {
            translated.push_str(" (translated to French)");
        } else if self.context.contains(&String::from("fr_to_en")) {
            translated.push_str(" (translated to English)");
        }
        translated
    }
}
