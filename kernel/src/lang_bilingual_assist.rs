extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn lang_bilingual_assist_init() {
    // Initialization logic for the module
}

pub extern "C" fn lang_bilingual_assist_exit() {
    // Cleanup logic for the module
}

pub struct BilingualAssistant {
    translations: Vec<(String, String)>,
}

impl BilingualAssistant {
    pub fn new() -> Self {
        BilingualAssistant {
            translations: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, from: &str, to: &str) {
        let from_str = String::from(from);
        let to_str = String::from(to);
        self.translations.push((from_str, to_str));
    }

    pub fn translate(&self, text: &str) -> Option<String> {
        for (from, to) in &self.translations {
            if from == text {
                return Some(to.clone());
            }
        }
        None
    }

    pub fn list_translations(&self) -> Vec<(String, String)> {
        self.translations.clone()
    }

    pub fn remove_translation(&mut self, text: &str) {
        self.translations.retain(|(from, _)| from != text);
    }
}

pub extern "C" fn lang_bilingual_assist_add_translation(from: *const u8, to: *const u8) {
    unsafe {
        let from_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(from, libc::strlen(from as *const i8)));
        let to_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(to, libc::strlen(to as *const i8)));

        let mut assistant = BilingualAssistant::new();
        assistant.add_translation(from_str, to_str);
    }
}

pub extern "C" fn lang_bilingual_assist_translate(text: *const u8) -> *mut u8 {
    unsafe {
        let text_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(text, libc::strlen(text as *const i8)));

        let assistant = BilingualAssistant::new();
        if let Some(translation) = assistant.translate(text_str) {
            let c_string = alloc::ffi::CString::new(translation).unwrap();
            return c_string.into_raw();
        }
    }
    core::ptr::null_mut()
}

pub extern "C" fn lang_bilingual_assist_free_translation(ptr: *mut u8) {
    unsafe {
        if !ptr.is_null() {
            alloc::ffi::CString::from_raw(ptr as *mut libc::c_char);
        }
    }
}
