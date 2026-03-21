extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_turkish_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_turkish_exit() {
    // Cleanup code for the module
}

pub struct TurkishAccentCorrector {
    corrections: Vec<(String, String)>,
}

impl TurkishAccentCorrector {
    pub fn new() -> Self {
        let mut corrections = Vec::new();
        corrections.push((String::from("i"), String::from("ı")));
        corrections.push((String::from("a"), String::from("â")));
        corrections.push((String::from("e"), String::from("ê")));
        corrections.push((String::from("o"), String::from("ö")));
        corrections.push((String::from("u"), String::from("ü")));
        TurkishAccentCorrector { corrections }
    }

    pub fn add_correction(&mut self, from: String, to: String) {
        self.corrections.push((from, to));
    }

    pub fn remove_correction(&mut self, from: &str) -> bool {
        let pos = self.corrections.iter().position(|(f, _)| f == from);
        if let Some(index) = pos {
            self.corrections.remove(index);
            true
        } else {
            false
        }
    }

    pub fn correct(&self, text: &str) -> String {
        let mut corrected_text = text.to_string();
        for (from, to) in &self.corrections {
            corrected_text = corrected_text.replace(from, to);
        }
        corrected_text
    }

    pub fn list_corrections(&self) -> Vec<(String, String)> {
        self.corrections.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accent_correction() {
        let mut corrector = TurkishAccentCorrector::new();
        assert_eq!(corrector.correct("i"), "ı");
        assert_eq!(corrector.correct("a"), "â");
        assert_eq!(corrector.correct("e"), "ê");
        assert_eq!(corrector.correct("o"), "ö");
        assert_eq!(corrector.correct("u"), "ü");

        corrector.add_correction(String::from("c"), String::from("ç"));
        assert_eq!(corrector.correct("c"), "ç");

        assert!(corrector.remove_correction("a"));
        assert!(!corrector.remove_correction("z"));

        let corrections = corrector.list_corrections();
        assert_eq!(corrections.len(), 5);
    }
}
