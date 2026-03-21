extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_portuguese_br_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_portuguese_br_exit() {
    // Cleanup logic for the module
}

pub struct PortugueseAccentCorrector {
    rules: Vec<(String, String)>,
}

impl PortugueseAccentCorrector {
    pub fn new() -> Self {
        let mut rules = Vec::new();
        // Add some basic accent correction rules
        rules.push((String::from("cafe"), String::from("café")));
        rules.push((String::from("teatro"), String::from("teátrio")));
        rules.push((String::from("mae"), String::from("mãe")));
        rules.push((String::from("pao"), String::from("pão")));
        rules.push((String::from("nao"), String::from("não")));

        PortugueseAccentCorrector { rules }
    }

    pub fn correct(&self, word: &str) -> String {
        let mut corrected = word.to_string();
        for (incorrect, correct) in &self.rules {
            if corrected == *incorrect {
                corrected = correct.clone();
                break;
            }
        }
        corrected
    }

    pub fn add_rule(&mut self, incorrect: String, correct: String) {
        self.rules.push((incorrect, correct));
    }

    pub fn remove_rule(&mut self, incorrect: &str) -> bool {
        let pos = self.rules.iter().position(|(inc, _)| inc == incorrect);
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

#[no_mangle]
pub extern "C" fn lang_accent_portuguese_br_correct(word: *const u8, len: usize) -> *mut u8 {
    let word_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(word, len)) };
    let corrector = PortugueseAccentCorrector::new();
    let corrected = corrector.correct(word_str);
    let result = alloc::ffi::CString::new(corrected).unwrap().into_raw();
    result
}

#[no_mangle]
pub extern "C" fn lang_accent_portuguese_br_free(ptr: *mut u8) {
    unsafe { alloc::ffi::CString::from_raw(ptr as *mut core::ffi::c_char) };
}
