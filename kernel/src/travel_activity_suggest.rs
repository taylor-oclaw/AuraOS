extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct TravelActivitySuggest {
    activities: Vec<String>,
}

impl TravelActivitySuggest {
    pub fn new() -> Self {
        TravelActivitySuggest {
            activities: vec![
                String::from("Hiking"),
                String::from("Camping"),
                String::from("Scuba Diving"),
                String::from("Mountain Biking"),
                String::from("Kayaking"),
            ],
        }
    }

    pub fn add_activity(&mut self, activity: &str) {
        self.activities.push(String::from(activity));
    }

    pub fn remove_activity(&mut self, activity: &str) -> bool {
        if let Some(index) = self.activities.iter().position(|a| a == activity) {
            self.activities.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_activities(&self) -> Vec<String> {
        self.activities.clone()
    }

    pub fn suggest_activity(&self, preference: &str) -> Option<&String> {
        self.activities.iter().find(|&activity| activity.contains(preference))
    }

    pub fn count_activities(&self) -> usize {
        self.activities.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let suggester = TravelActivitySuggest::new();
        assert_eq!(suggester.count_activities(), 5);
    }

    #[test]
    fn test_add_activity() {
        let mut suggester = TravelActivitySuggest::new();
        suggester.add_activity("Skydiving");
        assert_eq!(suggester.count_activities(), 6);
    }

    #[test]
    fn test_remove_activity() {
        let mut suggester = TravelActivitySuggest::new();
        assert!(suggester.remove_activity("Hiking"));
        assert_eq!(suggester.count_activities(), 4);
    }

    #[test]
    fn test_list_activities() {
        let suggester = TravelActivitySuggest::new();
        let activities = suggester.list_activities();
        assert_eq!(activities.len(), 5);
    }

    #[test]
    fn test_suggest_activity() {
        let suggester = TravelActivitySuggest::new();
        let suggestion = suggester.suggest_activity("Camping");
        assert_eq!(suggestion, Some(&String::from("Camping")));
    }
}
