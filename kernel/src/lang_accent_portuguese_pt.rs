extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn lang_accent_portuguese_pt_init() {
    // Initialization logic for the module
}

pub extern "C" fn lang_accent_portuguese_pt_exit() {
    // Cleanup logic for the module
}

pub struct PortugueseAccentCorrector {
    rules: Vec<(String, String)>,
}

impl PortugueseAccentCorrector {
    pub fn new() -> Self {
        let mut rules = Vec::new();
        // Example rules for accent correction
        rules.push((String::from("cafe"), String::from("café")));
        rules.push((String::from("mao"), String::from("mão")));
        rules.push((String::from("pao"), String::from("pão")));
        rules.push((String::from("nao"), String::from("não")));
        rules.push((String::from("casa"), String::from("cásar")));

        PortugueseAccentCorrector { rules }
    }

    pub fn correct(&self, word: &str) -> Option<String> {
        for (unaccented, accented) in &self.rules {
            if unaccented == word {
                return Some(accented.clone());
            }
        }
        None
    }

    pub fn add_rule(&mut self, unaccented: String, accented: String) {
        self.rules.push((unaccented, accented));
    }

    pub fn remove_rule(&mut self, unaccented: &str) -> bool {
        let pos = self.rules.iter().position(|(u, _)| u == unaccented);
        if let Some(index) = pos {
            self.rules.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_rules(&self) -> Vec<(String, String)> {
        self.rules.clone()
    }
}

pub extern "C" fn lang_accent_portuguese_pt_correct(word: *const u8, len: usize) -> *mut u8 {
    let word_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(word, len)) };
    let corrector = PortugueseAccentCorrector::new();
    if let Some(corrected) = corrector.correct(word_str) {
        let corrected_c_string = CString::new(corrected).unwrap();
        Box::into_raw(Box::new(corrected_c_string))
    } else {
        core::ptr::null_mut()
    }
}

pub extern "C" fn lang_accent_portuguese_pt_free(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe { Box::from_raw(ptr as *mut CString) };
    }
}
