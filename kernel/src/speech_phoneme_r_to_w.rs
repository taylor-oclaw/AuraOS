extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn speech_phoneme_r_to_w_init() {
    // Initialization logic for the module
}

pub extern "C" fn speech_phoneme_r_to_w_exit() {
    // Cleanup logic for the module
}

pub struct SpeechPhonemeRToW {
    phonemes: Vec<String>,
}

impl SpeechPhonemeRToW {
    pub fn new() -> Self {
        SpeechPhonemeRToW {
            phonemes: Vec::new(),
        }
    }

    pub fn add_phoneme(&mut self, phoneme: String) {
        self.phonemes.push(phoneme);
    }

    pub fn remove_phoneme(&mut self, index: usize) -> Option<String> {
        if index < self.phonemes.len() {
            Some(self.phonemes.remove(index))
        } else {
            None
        }
    }

    pub fn get_phoneme(&self, index: usize) -> Option<&String> {
        self.phonemes.get(index)
    }

    pub fn count_phonemes(&self) -> usize {
        self.phonemes.len()
    }

    pub fn list_phonemes(&self) -> Vec<String> {
        self.phonemes.clone()
    }
}

pub extern "C" fn speech_phoneme_r_to_w_add_phoneme(module: *mut SpeechPhonemeRToW, phoneme: *const u8, length: usize) {
    unsafe {
        if let Some(module_ref) = module.as_mut() {
            let phoneme_str = String::from_utf8_lossy(core::slice::from_raw_parts(phoneme, length)).to_string();
            module_ref.add_phoneme(phoneme_str);
        }
    }
}

pub extern "C" fn speech_phoneme_r_to_w_remove_phoneme(module: *mut SpeechPhonemeRToW, index: usize) -> Option<String> {
    unsafe {
        if let Some(module_ref) = module.as_mut() {
            module_ref.remove_phoneme(index)
        } else {
            None
        }
    }
}

pub extern "C" fn speech_phoneme_r_to_w_get_phoneme(module: *const SpeechPhonemeRToW, index: usize) -> Option<String> {
    unsafe {
        if let Some(module_ref) = module.as_ref() {
            module_ref.get_phoneme(index).cloned()
        } else {
            None
        }
    }
}

pub extern "C" fn speech_phoneme_r_to_w_count_phonemes(module: *const SpeechPhonemeRToW) -> usize {
    unsafe {
        if let Some(module_ref) = module.as_ref() {
            module_ref.count_phonemes()
        } else {
            0
        }
    }
}

pub extern "C" fn speech_phoneme_r_to_w_list_phonemes(module: *const SpeechPhonemeRToW, phonemes: *mut Vec<String>) {
    unsafe {
        if let Some(module_ref) = module.as_ref() {
            if let Some(phonemes_vec) = phonemes.as_mut() {
                *phonemes_vec = module_ref.list_phonemes();
            }
        }
    }
}
