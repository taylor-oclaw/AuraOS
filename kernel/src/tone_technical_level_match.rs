extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct ToneTechnicalLevelMatch {
    levels: Vec<String>,
}

impl ToneTechnicalLevelMatch {
    pub fn new(levels: Vec<String>) -> Self {
        ToneTechnicalLevelMatch { levels }
    }

    pub fn add_level(&mut self, level: String) {
        self.levels.push(level);
    }

    pub fn remove_level(&mut self, level: &str) -> bool {
        if let Some(index) = self.levels.iter().position(|x| x == level) {
            self.levels.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_levels(&self) -> Vec<String> {
        self.levels.clone()
    }

    pub fn find_level(&self, level: &str) -> Option<usize> {
        self.levels.iter().position(|x| x == level)
    }

    pub fn update_level(&mut self, old_level: &str, new_level: String) -> bool {
        if let Some(index) = self.levels.iter().position(|x| x == old_level) {
            self.levels[index] = new_level;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tone_technical_level_match() {
        let mut matcher = ToneTechnicalLevelMatch::new(vec![
            String::from("Beginner"),
            String::from("Intermediate"),
        ];

        assert_eq!(matcher.get_levels(), vec!["Beginner", "Intermediate"]);

        matcher.add_level(String::from("Advanced"));
        assert_eq!(
            matcher.get_levels(),
            vec!["Beginner", "Intermediate", "Advanced"]
        ;

        assert!(matcher.remove_level("Intermediate"));
        assert_eq!(matcher.get_levels(), vec!["Beginner", "Advanced"]);

        assert_eq!(matcher.find_level("Beginner"), Some(0));
        assert_eq!(matcher.find_level("Intermediate"), None);

        assert!(matcher.update_level("Beginner", String::from("Basic")));
        assert_eq!(matcher.get_levels(), vec!["Basic", "Advanced"]);
    }
}
