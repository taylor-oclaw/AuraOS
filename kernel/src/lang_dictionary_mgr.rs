extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn lang_dictionary_mgr_init() {
    // Initialization logic for the module
}

pub extern "C" fn lang_dictionary_mgr_exit() {
    // Cleanup logic for the module
}

pub struct LangDictionaryMgr {
    dictionary: Vec<(String, String)>,
}

impl LangDictionaryMgr {
    pub fn new() -> Self {
        LangDictionaryMgr {
            dictionary: Vec::new(),
        }
    }

    pub fn add_word(&mut self, key: &str, value: &str) {
        let key_str = String::from(key);
        let value_str = String::from(value);
        self.dictionary.push((key_str, value_str));
    }

    pub fn get_translation(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.dictionary {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove_word(&mut self, key: &str) {
        self.dictionary.retain(|(k, _)| k != key);
    }

    pub fn list_words(&self) -> Vec<&String> {
        self.dictionary.iter().map(|(k, _)| k).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_dictionary_mgr() {
        let mut mgr = LangDictionaryMgr::new();
        mgr.add_word("hello", "bonjour");
        assert_eq!(mgr.get_translation("hello"), Some(&String::from("bonjour")));
        mgr.remove_word("hello");
        assert_eq!(mgr.get_translation("hello"), None);
        mgr.add_word("world", "monde");
        let words = mgr.list_words();
        assert_eq!(words.len(), 1);
        assert_eq!(words[0], &String::from("world"));
    }
}
