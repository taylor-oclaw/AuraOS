extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct PrivacyLevelSet {
    levels: Vec<String>,
}

impl PrivacyLevelSet {
    pub fn new() -> Self {
        PrivacyLevelSet {
            levels: Vec::new(),
        }
    }

    pub fn add_level(&mut self, level: String) {
        if !self.levels.contains(&level) {
            self.levels.push(level);
        }
    }

    pub fn remove_level(&mut self, level: &str) -> bool {
        let pos = self.levels.iter().position(|x| x == level);
        match pos {
            Some(index) => {
                self.levels.remove(index);
                true
            }
            None => false,
        }
    }

    pub fn get_levels(&self) -> &[String] {
        &self.levels
    }

    pub fn has_level(&self, level: &str) -> bool {
        self.levels.contains(&String::from(level))
    }

    pub fn clear_levels(&mut self) {
        self.levels.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_level_set() {
        let mut set = PrivacyLevelSet::new();
        assert_eq!(set.get_levels().len(), 0);

        set.add_level(String::from("Low"));
        assert_eq!(set.get_levels().len(), 1);
        assert!(set.has_level("Low"));

        set.add_level(String::from("Medium"));
        assert_eq!(set.get_levels().len(), 2);
        assert!(set.has_level("Medium"));

        assert!(!set.remove_level("High"));
        assert_eq!(set.get_levels().len(), 2);

        assert!(set.remove_level("Low"));
        assert_eq!(set.get_levels().len(), 1);
        assert!(!set.has_level("Low"));

        set.clear_levels();
        assert_eq!(set.get_levels().len(), 0);
    }
}
