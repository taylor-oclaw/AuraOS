extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct ToneUrgencyMatch {
    keywords: Vec<String>,
    urgency_levels: Vec<u8>,
}

impl ToneUrgencyMatch {
    pub fn new(keywords: Vec<&str>, urgency_levels: Vec<u8>) -> Self {
        let keyword_strings = keywords.into_iter().map(String::from).collect();
        ToneUrgencyMatch {
            keywords: keyword_strings,
            urgency_levels,
        }
    }

    pub fn add_keyword(&mut self, keyword: &str, urgency_level: u8) {
        self.keywords.push(String::from(keyword));
        self.urgency_levels.push(urgency_level);
    }

    pub fn get_urgency_level(&self, text: &str) -> Option<u8> {
        for (keyword, &level) in self.keywords.iter().zip(self.urgency_levels.iter()) {
            if text.contains(keyword) {
                return Some(level);
            }
        }
        None
    }

    pub fn remove_keyword(&mut self, keyword: &str) {
        let pos = self.keywords.iter().position(|k| k == keyword);
        if let Some(index) = pos {
            self.keywords.remove(index);
            self.urgency_levels.remove(index);
        }
    }

    pub fn list_keywords(&self) -> Vec<String> {
        self.keywords.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tone_urgency_match() {
        let mut matcher = ToneUrgencyMatch::new(vec!["urgent", "important"], vec![5, 3]);
        assert_eq!(matcher.get_urgency_level("This is urgent"), Some(5));
        assert_eq!(matcher.get_urgency_level("This is important"), Some(3));
        assert_eq!(matcher.get_urgency_level("This is normal"), None);

        matcher.add_keyword("critical", 7);
        assert_eq!(matcher.get_urgency_level("This is critical"), Some(7));

        matcher.remove_keyword("important");
        assert_eq!(matcher.get_urgency_level("This is important"), None);

        let keywords = matcher.list_keywords();
        assert_eq!(keywords, vec!["urgent", "critical"]);
    }
}
