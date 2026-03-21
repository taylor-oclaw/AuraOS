extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct KeyboardLayout {
    keymap: Vec<Vec<char>>,
}

impl KeyboardLayout {
    pub fn new() -> Self {
        // Initialize a simple QWERTY layout for US keyboard
        let keymap = vec![
            vec!['`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\u{0}' ],
            vec!['\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\\' ],
            vec!['Caps Lock', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '\u{0}' ],
            vec!['Shift', '\\\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', 'Shift' ],
            vec!['Ctrl', 'Fn', 'Alt', 'Space', 'Alt', 'Fn', 'Ctrl'],
        ];

        KeyboardLayout { keymap }
    }

    pub fn get_key(&self, row: usize, col: usize) -> Option<char> {
        if let Some(row_vec) = self.keymap.get(row) {
            row_vec.get(col).cloned()
        } else {
            None
        }
    }

    pub fn get_row(&self, row: usize) -> Option<&Vec<char>> {
        self.keymap.get(row)
    }

    pub fn set_key(&mut self, row: usize, col: usize, key: char) {
        if let Some(row_vec) = self.keymap.get_mut(row) {
            if col < row_vec.len() {
                row_vec[col] = key;
            }
        }
    }

    pub fn add_row(&mut self, row: Vec<char>) {
        self.keymap.push(row);
    }

    pub fn remove_row(&mut self, row_index: usize) {
        if row_index < self.keymap.len() {
            self.keymap.remove(row_index);
        }
    }
}
