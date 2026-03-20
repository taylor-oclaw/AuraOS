extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    let mut output = StructuredOutput::new();
    output.add_entry("key1", "value1");
    output.add_entry("key2", "value2");
    output.update_entry("key1", "updated_value1");
    if let Some(value) = output.get_entry("key2") {
        // Do something with the value
    }
    let all_entries = output.get_all_entries();
    // Do something with all entries
    loop {}
}

pub struct StructuredOutput {
    entries: Vec<(String, String)>,
}

impl StructuredOutput {
    pub fn new() -> Self {
        StructuredOutput {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, key: &str, value: &str) {
        self.entries.push((String::from(key), String::from(value)));
    }

    pub fn update_entry(&mut self, key: &str, new_value: &str) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.0 == key) {
            entry.1 = String::from(new_value);
        }
    }

    pub fn get_entry(&self, key: &str) -> Option<&String> {
        self.entries.iter().find_map(|e| if e.0 == key { Some(&e.1) } else { None })
    }

    pub fn remove_entry(&mut self, key: &str) {
        self.entries.retain(|e| e.0 != key);
    }

    pub fn get_all_entries(&self) -> Vec<(&String, &String)> {
        self.entries.iter().map(|e| (&e.0, &e.1)).collect()
    }
}
