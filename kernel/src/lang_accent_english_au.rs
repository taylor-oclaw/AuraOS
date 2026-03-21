extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_english_au_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_english_au_exit() {
    // Cleanup code for the module
}

pub struct EnglishAccentAU {
    phrases: Vec<String>,
}

impl EnglishAccentAU {
    pub fn new() -> Self {
        EnglishAccentAU {
            phrases: vec![
                String::from("G'day mate!"),
                String::from("Crikey!"),
                String::from("Fair dinkum!"),
                String::from("Tucker time!"),
                String::from("Shazam!"),
            ],
        }
    }

    pub fn get_phrase(&self, index: usize) -> Option<&str> {
        self.phrases.get(index).map(|s| s.as_str())
    }

    pub fn add_phrase(&mut self, phrase: &str) {
        self.phrases.push(String::from(phrase));
    }

    pub fn remove_phrase(&mut self, index: usize) -> Option<String> {
        if index < self.phrases.len() {
            Some(self.phrases.remove(index))
        } else {
            None
        }
    }

    pub fn list_phrases(&self) -> Vec<&str> {
        self.phrases.iter().map(|s| s.as_str()).collect()
    }

    pub fn count_phrases(&self) -> usize {
        self.phrases.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let accent = EnglishAccentAU::new();
        assert_eq!(accent.count_phrases(), 5);
    }

    #[test]
    fn test_get_phrase() {
        let accent = EnglishAccentAU::new();
        assert_eq!(accent.get_phrase(0), Some("G'day mate!"));
        assert_eq!(accent.get_phrase(10), None);
    }

    #[test]
    fn test_add_phrase() {
        let mut accent = EnglishAccentAU::new();
        accent.add_phrase("How's it going?");
        assert_eq!(accent.count_phrases(), 6);
        assert_eq!(accent.get_phrase(5), Some("How's it going?"));
    }

    #[test]
    fn test_remove_phrase() {
        let mut accent = EnglishAccentAU::new();
        assert_eq!(accent.remove_phrase(0), Some(String::from("G'day mate!")));
        assert_eq!(accent.count_phrases(), 4);
        assert_eq!(accent.get_phrase(0), Some("Crikey!"));
    }

    #[test]
    fn test_list_phrases() {
        let accent = EnglishAccentAU::new();
        let phrases = accent.list_phrases();
        assert_eq!(phrases.len(), 5);
        assert_eq!(phrases[0], "G'day mate!");
        assert_eq!(phrases[1], "Crikey!");
    }

    #[test]
    fn test_count_phrases() {
        let accent = EnglishAccentAU::new();
        assert_eq!(accent.count_phrases(), 5);
    }
}
