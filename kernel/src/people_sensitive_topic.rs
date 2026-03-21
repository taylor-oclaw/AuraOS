extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn people_sensitive_topic_init() {
    // Initialization logic for the module
}

pub extern "C" fn people_sensitive_topic_exit() {
    // Cleanup logic for the module
}

pub struct PeopleSensitiveTopic {
    topics: Vec<String>,
}

impl PeopleSensitiveTopic {
    pub fn new() -> Self {
        PeopleSensitiveTopic {
            topics: Vec::new(),
        }
    }

    pub fn add_topic(&mut self, topic: String) {
        if !self.topics.contains(&topic) {
            self.topics.push(topic);
        }
    }

    pub fn remove_topic(&mut self, topic: &str) -> bool {
        let index = self.topics.iter().position(|t| t == topic);
        if let Some(i) = index {
            self.topics.remove(i);
            true
        } else {
            false
        }
    }

    pub fn contains_topic(&self, topic: &str) -> bool {
        self.topics.contains(&String::from(topic))
    }

    pub fn list_topics(&self) -> Vec<String> {
        self.topics.clone()
    }

    pub fn count_topics(&self) -> usize {
        self.topics.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_people_sensitive_topic() {
        let mut pst = PeopleSensitiveTopic::new();
        assert_eq!(pst.count_topics(), 0);

        pst.add_topic(String::from("politics"));
        assert_eq!(pst.count_topics(), 1);
        assert!(pst.contains_topic("politics"));

        pst.add_topic(String::from("religion"));
        assert_eq!(pst.count_topics(), 2);
        assert!(pst.contains_topic("religion"));

        let topics = pst.list_topics();
        assert_eq!(topics.len(), 2);
        assert!(topics.contains(&String::from("politics")));
        assert!(topics.contains(&String::from("religion")));

        assert!(pst.remove_topic("politics"));
        assert!(!pst.contains_topic("politics"));
        assert_eq!(pst.count_topics(), 1);

        assert!(!pst.remove_topic("science"));
        assert_eq!(pst.count_topics(), 1);
    }
}
