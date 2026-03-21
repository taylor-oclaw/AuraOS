extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_std]
mod lang_code_switch_detect {
    use core::cmp::Ordering;

    pub struct LanguageCodeSwitchDetector {
        previous_language: Option<String>,
        language_history: Vec<String>,
    }

    impl LanguageCodeSwitchDetector {
        pub fn new() -> Self {
            LanguageCodeSwitchDetector {
                previous_language: None,
                language_history: Vec::new(),
            }
        }

        pub fn detect_language(&mut self, current_language: &str) {
            let lang = String::from(current_language);
            if let Some(ref prev_lang) = self.previous_language {
                if prev_lang != &lang {
                    self.language_history.push(lang.clone());
                }
            } else {
                self.language_history.push(lang.clone());
            }
            self.previous_language = Some(lang);
        }

        pub fn get_previous_language(&self) -> Option<&String> {
            self.previous_language.as_ref()
        }

        pub fn get_language_history(&self) -> &Vec<String> {
            &self.language_history
        }

        pub fn clear_history(&mut self) {
            self.language_history.clear();
        }

        pub fn compare_languages(&self, lang1: &str, lang2: &str) -> Ordering {
            String::from(lang1).cmp(&String::from(lang2))
        }
    }
}
