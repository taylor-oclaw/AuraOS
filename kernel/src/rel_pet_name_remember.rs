extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rel_pet_name_remember_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_pet_name_remember_exit() {
    // Cleanup logic for the module
}

pub struct PetNameRememberer {
    names: Vec<String>,
}

impl PetNameRememberer {
    pub fn new() -> Self {
        PetNameRememberer {
            names: Vec::new(),
        }
    }

    pub fn add_name(&mut self, name: String) {
        if !self.names.contains(&name) {
            self.names.push(name);
        }
    }

    pub fn remove_name(&mut self, name: &str) -> bool {
        let pos = self.names.iter().position(|n| n == name);
        if let Some(index) = pos {
            self.names.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_names(&self) -> Vec<String> {
        self.names.clone()
    }

    pub fn has_name(&self, name: &str) -> bool {
        self.names.contains(&String::from(name))
    }

    pub fn count_names(&self) -> usize {
        self.names.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pet_name_rememberer() {
        let mut rememberer = PetNameRememberer::new();
        assert_eq!(rememberer.count_names(), 0);

        rememberer.add_name(String::from("Buddy"));
        assert_eq!(rememberer.count_names(), 1);
        assert!(rememberer.has_name("Buddy"));

        rememberer.add_name(String::from("Charlie"));
        assert_eq!(rememberer.count_names(), 2);
        assert!(rememberer.has_name("Charlie"));

        rememberer.remove_name("Buddy");
        assert_eq!(rememberer.count_names(), 1);
        assert!(!rememberer.has_name("Buddy"));

        let names = rememberer.get_names();
        assert_eq!(names, vec![String::from("Charlie")]);
    }
}
