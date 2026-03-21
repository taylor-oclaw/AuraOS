extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_sign_language_jsl_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_sign_language_jsl_exit() {
    // Cleanup logic for the module
}

pub struct SpeechSignLanguageJSL {
    translations: Vec<(String, String)>,
}

impl SpeechSignLanguageJSL {
    pub fn new() -> Self {
        SpeechSignLanguageJSL {
            translations: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, speech: &str, sign_language: &str) {
        let speech = String::from(speech);
        let sign_language = String::from(sign_language);
        self.translations.push((speech, sign_language));
    }

    pub fn get_sign_language(&self, speech: &str) -> Option<&String> {
        for (s, sl) in &self.translations {
            if s == speech {
                return Some(sl);
            }
        }
        None
    }

    pub fn remove_translation(&mut self, speech: &str) {
        self.translations.retain(|(s, _)| s != speech);
    }

    pub fn list_translations(&self) -> Vec<&String> {
        self.translations.iter().map(|(_, sl)| sl).collect()
    }
}

#[no_mangle]
pub extern "C" fn speech_sign_language_jsl_add_translation(module: *mut SpeechSignLanguageJSL, speech: *const u8, sign_language: *const u8) {
    unsafe {
        if let Some(module) = module.as_mut() {
            let speech_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(speech, libc::strlen(speech as *const _) + 1));
            let sign_language_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(sign_language, libc::strlen(sign_language as *const _) + 1));
            module.add_translation(speech_str, sign_language_str);
        }
    }
}

#[no_mangle]
pub extern "C" fn speech_sign_language_jsl_get_sign_language(module: *mut SpeechSignLanguageJSL, speech: *const u8) -> *const u8 {
    unsafe {
        if let Some(module) = module.as_mut() {
            let speech_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(speech, libc::strlen(speech as *const _) + 1));
            if let Some(sign_language) = module.get_sign_language(speech_str) {
                return sign_language.as_ptr();
            }
        }
        core::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn speech_sign_language_jsl_remove_translation(module: *mut SpeechSignLanguageJSL, speech: *const u8) {
    unsafe {
        if let Some(module) = module.as_mut() {
            let speech_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(speech, libc::strlen(speech as *const _) + 1));
            module.remove_translation(speech_str);
        }
    }
}

#[no_mangle]
pub extern "C" fn speech_sign_language_jsl_list_translations(module: *mut SpeechSignLanguageJSL) -> Vec<*const u8> {
    unsafe {
        if let Some(module) = module.as_mut() {
            return module.list_translations().into_iter().map(|sl| sl.as_ptr()).collect();
        }
        Vec::new()
    }
}
