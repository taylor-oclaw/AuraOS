extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OnScreenKeyboard {
    keys: Vec<String>,
    current_key_index: usize,
}

impl OnScreenKeyboard {
    pub fn new() -> Self {
        let keys = vec![
            String::from("Q"), String::from("W"), String::from("E"), String::from("R"),
            String::from("T"), String::from("Y"), String::from("U"), String::from("I"),
            String::from("O"), String::from("P"), String::from("A"), String::from("S"),
            String::from("D"), String::from("F"), String::from("G"), String::from("H"),
            String::from("J"), String::from("K"), String::from("L"), String::from("Z"),
            String::from("X"), String::from("C"), String::from("V"), String::from("B"),
            String::from("N"), String::from("M"),
        ];
        OnScreenKeyboard {
            keys,
            current_key_index: 0,
        }
    }

    pub fn get_current_key(&self) -> &str {
        &self.keys[self.current_key_index]
    }

    pub fn move_right(&mut self) {
        if self.current_key_index < self.keys.len() - 1 {
            self.current_key_index += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.current_key_index > 0 {
            self.current_key_index -= 1;
        }
    }

    pub fn select_key(&self) -> &str {
        &self.keys[self.current_key_index]
    }

    pub fn get_keys(&self) -> &[String] {
        &self.keys
    }
}
