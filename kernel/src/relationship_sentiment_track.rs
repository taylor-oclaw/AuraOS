extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod relationship_sentiment_track {
    use super::*;

    pub struct RelationshipSentimentTracker {
        relationships: Vec<(String, String, i32)>,
    }

    impl RelationshipSentimentTracker {
        pub fn new() -> Self {
            RelationshipSentimentTracker {
                relationships: Vec::new(),
            }
        }

        pub fn add_relationship(&mut self, person1: &str, person2: &str, sentiment: i32) {
            self.relationships.push((String::from(person1), String::from(person2), sentiment));
        }

        pub fn get_sentiment(&self, person1: &str, person2: &str) -> Option<i32> {
            for (p1, p2, sentiment) in &self.relationships {
                if p1 == person1 && p2 == person2 {
                    return Some(*sentiment);
                }
            }
            None
        }

        pub fn update_sentiment(&mut self, person1: &str, person2: &str, new_sentiment: i32) -> bool {
            for (p1, p2, sentiment) in &mut self.relationships {
                if p1 == person1 && p2 == person2 {
                    *sentiment = new_sentiment;
                    return true;
                }
            }
            false
        }

        pub fn remove_relationship(&mut self, person1: &str, person2: &str) -> bool {
            let pos = self.relationships.iter().position(|(p1, p2, _)| p1 == person1 && p2 == person2);
            if let Some(index) = pos {
                self.relationships.remove(index);
                true
            } else {
                false
            }
        }

        pub fn list_relationships(&self) -> Vec<(String, String, i32)> {
            self.relationships.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::relationship_sentiment_track::*;

    #[test]
    fn test_relationship_sentiment_tracker() {
        let mut tracker = RelationshipSentimentTracker::new();
        tracker.add_relationship("Alice", "Bob", 10);
        assert_eq!(tracker.get_sentiment("Alice", "Bob"), Some(10));
        assert!(tracker.update_sentiment("Alice", "Bob", 20));
        assert_eq!(tracker.get_sentiment("Alice", "Bob"), Some(20));
        assert!(tracker.remove_relationship("Alice", "Bob"));
        assert_eq!(tracker.get_sentiment("Alice", "Bob"), None);
    }
}
