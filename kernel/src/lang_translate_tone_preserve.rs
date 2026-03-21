extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_translate_tone_preserve_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_translate_tone_preserve_exit() {
    // Cleanup logic for the module
}

pub struct LangTranslateTonePreserve {
    translations: Vec<(String, String)>,
}

impl LangTranslateTonePreserve {
    pub fn new() -> Self {
        LangTranslateTonePreserve {
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
            if text == from {
                return Some(to.clone());
            }
        }
        None
    }

    pub fn remove_translation(&mut self, from: &str) {
        self.translations.retain(|(f, _)| f != from);
    }

    pub fn list_translations(&self) -> Vec<(String, String)> {
        self.translations.clone()
    }
}

#[no_mangle]
pub extern "C" fn lang_translate_tone_preserve_add_translation(module: *mut LangTranslateTonePreserve, from: *const u8, to: *const u8) {
    unsafe {
        if let Some(module_ref) = module.as_mut() {
            let from_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(from, libc::strlen(from as *const _) + 1));
            let to_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(to, libc::strlen(to as *const _) + 1));
            module_ref.add_translation(from_str, to_str);
        }
    }
}

#[no_mangle]
pub extern "C" fn lang_translate_tone_preserve_translate(module: *const LangTranslateTonePreserve, text: *const u8) -> *mut u8 {
    unsafe {
        if let Some(module_ref) = module.as_ref() {
            let text_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(text, libc::strlen(text as *const _) + 1));
            if let Some(translated) = module_ref.translate(text_str) {
                return translated.into_boxed_str().into_raw() as *mut u8;
            }
        }
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn lang_translate_tone_preserve_remove_translation(module: *mut LangTranslateTonePreserve, from: *const u8) {
    unsafe {
        if let Some(module_ref) = module.as_mut() {
            let from_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(from, libc::strlen(from as *const _) + 1));
            module_ref.remove_translation(from_str);
        }
    }
}

#[no_mangle]
pub extern "C" fn lang_translate_tone_preserve_list_translations(module: *const LangTranslateTonePreserve) -> *mut Vec<(String, String)> {
    unsafe {
        if let Some(module_ref) = module.as_ref() {
            return Box::into_raw(Box::new(module_ref.list_translations()));
        }
    }
    core::ptr::null_mut()
}
