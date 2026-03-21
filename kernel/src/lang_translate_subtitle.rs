extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangTranslateSubtitle {
    subtitles: Vec<String>,
}

impl LangTranslateSubtitle {
    pub fn new() -> Self {
        LangTranslateSubtitle {
            subtitles: Vec::new(),
        }
    }

    pub fn add_subtitle(&mut self, subtitle: String) {
        self.subtitles.push(subtitle);
    }

    pub fn get_subtitle(&self, index: usize) -> Option<&String> {
        self.subtitles.get(index)
    }

    pub fn remove_subtitle(&mut self, index: usize) -> Option<String> {
        if index < self.subtitles.len() {
            Some(self.subtitles.remove(index))
        } else {
            None
        }
    }

    pub fn translate_subtitle(&self, index: usize, target_language: &str) -> Option<String> {
        // Dummy translation logic for demonstration purposes
        match self.get_subtitle(index) {
            Some(subtitle) => {
                let translated = format!("{} ({})", subtitle, target_language);
                Some(translated)
            }
            None => None,
        }
    }

    pub fn list_subtitles(&self) -> Vec<&String> {
        self.subtitles.iter().collect()
    }
}
