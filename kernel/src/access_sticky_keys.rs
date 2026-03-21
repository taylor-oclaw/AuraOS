extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct StickyKeys {
    keys: Vec<String>,
}

impl StickyKeys {
    pub fn new() -> Self {
        StickyKeys { keys: Vec::new() }
    }

    pub fn add_key(&mut self, key: &str) {
        self.keys.push(String::from(key));
    }

    pub fn remove_key(&mut self, key: &str) {
        if let Some(index) = self.keys.iter().position(|k| k == key) {
            self.keys.remove(index);
        }
    }

    pub fn has_key(&self, key: &str) -> bool {
        self.keys.contains(&String::from(key))
    }

    pub fn list_keys(&self) -> Vec<String> {
        self.keys.clone()
    }

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sticky_keys() {
        let mut sticky_keys = StickyKeys::new();

        assert_eq!(sticky_keys.list_keys(), Vec::<String>::new());

        sticky_keys.add_key("Ctrl");
        sticky_keys.add_key("Shift");

        assert_eq!(sticky_keys.has_key("Ctrl"), true);
        assert_eq!(sticky_keys.has_key("Alt"), false);

        let keys = sticky_keys.list_keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&String::from("Ctrl")));
        assert!(keys.contains(&String::from("Shift")));

        sticky_keys.remove_key("Ctrl");
        assert_eq!(sticky_keys.has_key("Ctrl"), false);

        sticky_keys.clear_keys();
        assert_eq!(sticky_keys.list_keys(), Vec::<String>::new());
    }
}
