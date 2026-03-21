extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_french_ca_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_french_ca_exit() {
    // Cleanup code for the module
}

pub struct FrenchCanadianAccent {
    words: Vec<String>,
}

impl FrenchCanadianAccent {
    pub fn new() -> Self {
        FrenchCanadianAccent {
            words: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        self.words.push(word);
    }

    pub fn get_words(&self) -> &Vec<String> {
        &self.words
    }

    pub fn remove_word(&mut self, index: usize) -> Option<String> {
        if index < self.words.len() {
            Some(self.words.remove(index))
        } else {
            None
        }
    }

    pub fn replace_word(&mut self, index: usize, new_word: String) -> bool {
        if index < self.words.len() {
            self.words[index] = new_word;
            true
        } else {
            false
        }
    }

    pub fn count_words(&self) -> usize {
        self.words.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_french_canadian_accent() {
        let mut accent = FrenchCanadianAccent::new();
        assert_eq!(accent.count_words(), 0);

        accent.add_word(String::from("bonjour"));
        accent.add_word(String::from("comment"));
        assert_eq!(accent.count_words(), 2);
        assert_eq!(accent.get_words()[0], "bonjour");
        assert_eq!(accent.get_words()[1], "comment");

        let removed = accent.remove_word(0);
        assert_eq!(removed, Some(String::from("bonjour")));
        assert_eq!(accent.count_words(), 1);

        let replaced = accent.replace_word(0, String::from("salut"));
        assert_eq!(replaced, true);
        assert_eq!(accent.get_words()[0], "salut");

        let replaced_out_of_bounds = accent.replace_word(2, String::from("test"));
        assert_eq!(replaced_out_of_bounds, false);
    }
}
