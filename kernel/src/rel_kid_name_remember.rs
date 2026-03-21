extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_kid_name_remember_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn rel_kid_name_remember_exit() {
    // Cleanup code for the module
}

pub struct KidNameRemember {
    names: Vec<String>,
}

impl KidNameRemember {
    pub fn new() -> Self {
        KidNameRemember {
            names: Vec::new(),
        }
    }

    pub fn add_name(&mut self, name: &str) {
        self.names.push(String::from(name));
    }

    pub fn remove_name(&mut self, name: &str) {
        if let Some(index) = self.names.iter().position(|n| n == name) {
            self.names.remove(index);
        }
    }

    pub fn get_names(&self) -> &[String] {
        &self.names
    }

    pub fn contains_name(&self, name: &str) -> bool {
        self.names.contains(&String::from(name))
    }

    pub fn clear_names(&mut self) {
        self.names.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kid_name_remember() {
        let mut remember = KidNameRemember::new();
        assert_eq!(remember.get_names().len(), 0);

        remember.add_name("Alice");
        remember.add_name("Bob");
        assert_eq!(remember.get_names().len(), 2);
        assert!(remember.contains_name("Alice"));
        assert!(!remember.contains_name("Charlie"));

        remember.remove_name("Alice");
        assert_eq!(remember.get_names().len(), 1);
        assert!(!remember.contains_name("Alice"));

        remember.clear_names();
        assert_eq!(remember.get_names().len(), 0);
    }
}
