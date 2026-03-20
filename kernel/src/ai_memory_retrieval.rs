extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct MemoryRetrieval {
    data: Vec<String>,
}

impl MemoryRetrieval {
    pub fn new() -> Self {
        MemoryRetrieval { data: Vec::new() }
    }

    pub fn add_data(&mut self, item: String) {
        self.data.push(item);
    }

    pub fn get_data(&self, index: usize) -> Option<&String> {
        self.data.get(index)
    }

    pub fn remove_data(&mut self, index: usize) -> Option<String> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn find_data(&self, query: &str) -> Vec<&String> {
        self.data.iter().filter(|&&item| item.contains(query)).collect()
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_retrieval() {
        let mut mr = MemoryRetrieval::new();
        assert_eq!(mr.get_data(0), None);

        mr.add_data(String::from("hello"));
        mr.add_data(String::from("world"));

        assert_eq!(mr.get_data(0), Some(&String::from("hello")));
        assert_eq!(mr.get_data(1), Some(&String::from("world")));

        let removed = mr.remove_data(0);
        assert_eq!(removed, Some(String::from("hello")));
        assert_eq!(mr.get_data(0), Some(&String::from("world")));

        let found = mr.find_data("or");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0], &String::from("world"));

        mr.clear_data();
        assert_eq!(mr.get_data(0), None);
    }
}
