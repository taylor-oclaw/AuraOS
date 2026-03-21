extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct SpeechImpedimentProfile {
    name: String,
    age: u32,
    impediments: Vec<String>,
    treatments: Vec<String>,
    severity_level: u8, // 1 to 5
}

impl SpeechImpedimentProfile {
    pub fn new(name: &str, age: u32) -> Self {
        SpeechImpedimentProfile {
            name: String::from(name),
            age,
            impediments: Vec::new(),
            treatments: Vec::new(),
            severity_level: 1,
        }
    }

    pub fn add_impediment(&mut self, impediment: &str) {
        self.impediments.push(String::from(impediment));
    }

    pub fn remove_impediment(&mut self, impediment: &str) -> bool {
        if let Some(index) = self.impediments.iter().position(|i| i == impediment) {
            self.impediments.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_treatment(&mut self, treatment: &str) {
        self.treatments.push(String::from(treatment));
    }

    pub fn remove_treatment(&mut self, treatment: &str) -> bool {
        if let Some(index) = self.treatments.iter().position(|t| t == treatment) {
            self.treatments.remove(index);
            true
        } else {
            false
        }
    }

    pub fn update_severity_level(&mut self, level: u8) {
        if level >= 1 && level <= 5 {
            self.severity_level = level;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speech_impediment_profile() {
        let mut profile = SpeechImpedimentProfile::new("John Doe", 30);
        assert_eq!(profile.name, "John Doe");
        assert_eq!(profile.age, 30);
        assert!(profile.impediments.is_empty());
        assert!(profile.treatments.is_empty());
        assert_eq!(profile.severity_level, 1);

        profile.add_impediment("Stuttering");
        assert_eq!(profile.impediments.len(), 1);
        assert_eq!(profile.impediments[0], "Stuttering");

        let removed = profile.remove_impediment("Stuttering");
        assert!(removed);
        assert!(profile.impediments.is_empty());

        profile.add_treatment("Speech Therapy");
        assert_eq!(profile.treatments.len(), 1);
        assert_eq!(profile.treatments[0], "Speech Therapy");

        let removed = profile.remove_treatment("Speech Therapy");
        assert!(removed);
        assert!(profile.treatments.is_empty());

        profile.update_severity_level(3);
        assert_eq!(profile.severity_level, 3);

        profile.update_severity_level(6); // Invalid level
        assert_eq!(profile.severity_level, 3);
    }
}
