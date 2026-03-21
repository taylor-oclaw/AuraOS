extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_allergy_remember_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rel_allergy_remember_exit() {
    // Cleanup logic for the module
}

pub struct AllergyRecord {
    name: String,
    symptoms: Vec<String>,
    severity: u8, // 1 to 5 scale
}

impl AllergyRecord {
    pub fn new(name: &str, symptoms: &[&str], severity: u8) -> Self {
        AllergyRecord {
            name: String::from(name),
            symptoms: symptoms.iter().map(|s| String::from(*s)).collect(),
            severity,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_symptom(&mut self, symptom: &str) {
        if !self.symptoms.contains(&String::from(symptom)) {
            self.symptoms.push(String::from(symptom));
        }
    }

    pub fn remove_symptom(&mut self, symptom: &str) {
        self.symptoms.retain(|s| s != symptom);
    }

    pub fn get_severity(&self) -> u8 {
        self.severity
    }

    pub fn set_severity(&mut self, severity: u8) {
        if severity > 0 && severity <= 5 {
            self.severity = severity;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allergy_record() {
        let mut record = AllergyRecord::new("Peanut", &["Hives", "Swelling"], 3);

        assert_eq!(record.get_name(), "Peanut");
        assert_eq!(record.get_severity(), 3);
        assert_eq!(record.symptoms, vec![String::from("Hives"), String::from("Swelling")]);

        record.add_symptom("Anaphylaxis");
        assert_eq!(
            record.symptoms,
            vec![
                String::from("Hives"),
                String::from("Swelling"),
                String::from("Anaphylaxis")
            ]
        );

        record.remove_symptom("Hives");
        assert_eq!(record.symptoms, vec![String::from("Swelling"), String::from("Anaphylaxis")]);

        record.set_severity(5);
        assert_eq!(record.get_severity(), 5);

        record.set_severity(0); // Invalid severity
        assert_eq!(record.get_severity(), 5);
    }
}
