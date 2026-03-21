extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let keyboard = AccessOneHandKeyboard::new();
    keyboard.init();
    loop {}
}

pub struct AccessOneHandKeyboard {
    keys: Vec<String>,
    current_key_index: usize,
}

impl AccessOneHandKeyboard {
    pub fn new() -> Self {
        AccessOneHandKeyboard {
            keys: vec![
                String::from("a"),
                String::from("s"),
                String::from("d"),
                String::from("f"),
                String::from("g"),
                String::from("h"),
                String::from("j"),
                String::from("k"),
                String::from("l"),
            ],
            current_key_index: 0,
        }
    }

    pub fn init(&mut self) {
        // Initialization logic for the keyboard
    }

    pub fn next_key(&mut self) -> Option<&str> {
        if self.current_key_index < self.keys.len() {
            let key = &self.keys[self.current_key_index];
            self.current_key_index += 1;
            Some(key)
        } else {
            None
        }
    }

    pub fn previous_key(&mut self) -> Option<&str> {
        if self.current_key_index > 0 {
            self.current_key_index -= 1;
            Some(&self.keys[self.current_key_index])
        } else {
            None
        }
    }

    pub fn get_current_key(&self) -> Option<&str> {
        self.keys.get(self.current_key_index).map(|s| s.as_str())
    }

    pub fn reset(&mut self) {
        self.current_key_index = 0;
    }
}
