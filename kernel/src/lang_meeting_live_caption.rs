extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct LiveCaption {
    captions: Vec<String>,
    current_caption: String,
}

impl LiveCaption {
    pub fn new() -> Self {
        LiveCaption {
            captions: Vec::new(),
            current_caption: String::new(),
        }
    }

    pub fn add_caption(&mut self, caption: &str) {
        let caption_str = String::from(caption);
        self.captions.push(caption_str);
    }

    pub fn get_current_caption(&self) -> &str {
        &self.current_caption
    }

    pub fn set_current_caption(&mut self, index: usize) -> Result<(), &'static str> {
        if let Some(caption) = self.captions.get(index) {
            self.current_caption = caption.clone();
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn clear_captions(&mut self) {
        self.captions.clear();
        self.current_caption.clear();
    }

    pub fn get_all_captions(&self) -> &[String] {
        &self.captions
    }
}
