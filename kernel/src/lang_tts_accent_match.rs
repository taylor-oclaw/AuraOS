extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_tts_accent_match_init() {
    // Initialization code if needed
}

#[no_mangle]
pub extern "C" fn lang_tts_accent_match_exit() {
    // Cleanup code if needed
}

pub struct AccentMatcher {
    accents: Vec<String>,
}

impl AccentMatcher {
    pub fn new() -> Self {
        AccentMatcher {
            accents: Vec::new(),
        }
    }

    pub fn add_accent(&mut self, accent: String) {
        self.accents.push(accent);
    }

    pub fn remove_accent(&mut self, accent: &str) -> bool {
        if let Some(index) = self.accents.iter().position(|a| a == accent) {
            self.accents.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_accent(&self, accent: &str) -> bool {
        self.accents.contains(&String::from(accent))
    }

    pub fn list_accents(&self) -> Vec<String> {
        self.accents.clone()
    }

    pub fn count_accents(&self) -> usize {
        self.accents.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accent_matcher() {
        let mut matcher = AccentMatcher::new();
        assert_eq!(matcher.count_accents(), 0);

        matcher.add_accent(String::from("British"));
        matcher.add_accent(String::from("American"));

        assert_eq!(matcher.count_accents(), 2);
        assert!(matcher.has_accent("British"));
        assert!(!matcher.has_accent("Australian"));

        let accents = matcher.list_accents();
        assert_eq!(accents.len(), 2);
        assert!(accents.contains(&String::from("British")));
        assert!(accents.contains(&String::from("American")));

        assert!(matcher.remove_accent("British"));
        assert!(!matcher.has_accent("British"));
        assert_eq!(matcher.count_accents(), 1);

        assert!(!matcher.remove_accent("Australian"));
    }
}
