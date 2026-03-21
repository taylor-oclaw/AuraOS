extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_hyponasal_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_hyponasal_exit() {
    // Cleanup logic for the module
}

pub struct SpeechHyponasal {
    data: Vec<u8>,
    buffer: String,
}

impl SpeechHyponasal {
    pub fn new() -> Self {
        SpeechHyponasal {
            data: Vec::new(),
            buffer: String::new(),
        }
    }

    pub fn add_data(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn append_to_buffer(&mut self, text: &str) {
        self.buffer.push_str(text);
    }

    pub fn get_buffer_content(&self) -> &str {
        &self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speech_hyponasal() {
        let mut sh = SpeechHyponasal::new();
        assert!(sh.get_data().is_empty());
        assert_eq!(sh.get_buffer_content(), "");

        sh.add_data(b"Hello");
        assert_eq!(sh.get_data(), b"Hello");

        sh.append_to_buffer(" World");
        assert_eq!(sh.get_buffer_content(), " World");

        sh.clear_data();
        assert!(sh.get_data().is_empty());
    }
}
