extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

pub struct ClipboardHistory {
    history: Vec<String>,
    max_entries: usize,
}

impl ClipboardHistory {
    pub fn new(max_entries: usize) -> Self {
        ClipboardHistory {
            history: Vec::new(),
            max_entries,
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        if self.history.len() >= self.max_entries {
            self.history.remove(0);
        }
        self.history.push(entry);
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn get_last_entry(&self) -> Option<&String> {
        self.history.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_history() {
        let mut history = ClipboardHistory::new(3);
        history.add_entry(String::from("Entry 1"));
        history.add_entry(String::from("Entry 2"));
        history.add_entry(String::from("Entry 3"));

        assert_eq!(history.get_history().len(), 3);
        assert_eq!(history.get_last_entry(), Some(&String::from("Entry 3")));

        history.add_entry(String::from("Entry 4"));
        assert_eq!(history.get_history().len(), 3);
        assert_eq!(history.get_last_entry(), Some(&String::from("Entry 4")));
        assert_eq!(history.get_history()[0], String::from("Entry 2"));

        history.clear_history();
        assert_eq!(history.get_history().len(), 0);
    }
}
