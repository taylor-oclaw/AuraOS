extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_output_language_select_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_output_language_select_exit() {
    // Cleanup logic for the module
}

pub struct LanguageSelector {
    languages: Vec<String>,
    selected_language: Option<usize>,
}

impl LanguageSelector {
    pub fn new(languages: Vec<&str>) -> Self {
        let language_strings: Vec<String> = languages.into_iter().map(String::from).collect();
        LanguageSelector {
            languages: language_strings,
            selected_language: None,
        }
    }

    pub fn add_language(&mut self, language: &str) {
        self.languages.push(String::from(language));
    }

    pub fn remove_language(&mut self, index: usize) -> Option<String> {
        if index < self.languages.len() {
            Some(self.languages.remove(index))
        } else {
            None
        }
    }

    pub fn select_language(&mut self, index: usize) -> Result<(), &'static str> {
        if index < self.languages.len() {
            self.selected_language = Some(index);
            Ok(())
        } else {
            Err("Language index out of bounds")
        }
    }

    pub fn get_selected_language(&self) -> Option<&str> {
        self.selected_language.map(|index| &self.languages[index])
    }

    pub fn list_languages(&self) -> Vec<&str> {
        self.languages.iter().map(|lang| lang.as_str()).collect()
    }
}

#[no_mangle]
pub extern "C" fn create_language_selector(languages: *const *const u8, count: usize) -> *mut LanguageSelector {
    let mut language_vec = Vec::new();
    for i in 0..count {
        unsafe {
            let lang_ptr = languages.offset(i as isize);
            let lang_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(*lang_ptr, libc::strlen(*lang_ptr)));
            language_vec.push(lang_str);
        }
    }
    Box::into_raw(Box::new(LanguageSelector::new(language_vec)))
}

#[no_mangle]
pub extern "C" fn destroy_language_selector(selector: *mut LanguageSelector) {
    if !selector.is_null() {
        unsafe { drop(Box::from_raw(selector)); }
    }
}
