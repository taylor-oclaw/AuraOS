extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct PeopleChannelPreference {
    preferences: Vec<(String, String)>,
}

impl PeopleChannelPreference {
    pub fn new() -> Self {
        PeopleChannelPreference {
            preferences: Vec::new(),
        }
    }

    pub fn add_preference(&mut self, person: &str, channel: &str) {
        let person = String::from(person);
        let channel = String::from(channel);
        self.preferences.push((person, channel));
    }

    pub fn get_channels_for_person(&self, person: &str) -> Vec<String> {
        self.preferences
            .iter()
            .filter(|&&(ref p, _)| p == person)
            .map(|(_, ref c)| c.clone())
            .collect()
    }

    pub fn get_people_for_channel(&self, channel: &str) -> Vec<String> {
        self.preferences
            .iter()
            .filter(|&&(_, ref c)| c == channel)
            .map(|(ref p, _)| p.clone())
            .collect()
    }

    pub fn remove_preference(&mut self, person: &str, channel: &str) {
        self.preferences.retain(|&(ref p, ref c)| !(p == person && c == channel));
    }

    pub fn list_all_preferences(&self) -> Vec<(String, String)> {
        self.preferences.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_people_channel_preference() {
        let mut preference = PeopleChannelPreference::new();
        preference.add_preference("Alice", "Email");
        preference.add_preference("Bob", "SMS");
        preference.add_preference("Alice", "Push");

        assert_eq!(
            preference.get_channels_for_person("Alice"),
            vec![String::from("Email"), String::from("Push")]
        );
        assert_eq!(
            preference.get_people_for_channel("SMS"),
            vec![String::from("Bob")]
        );

        preference.remove_preference("Alice", "Email");
        assert_eq!(
            preference.get_channels_for_person("Alice"),
            vec![String::from("Push")]
        );

        let all_preferences = preference.list_all_preferences();
        assert_eq!(all_preferences.len(), 2);
    }
}
