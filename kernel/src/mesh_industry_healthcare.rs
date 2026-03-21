extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_industry_healthcare_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_industry_healthcare_exit() {
    // Cleanup logic for the module
}

pub struct HealthcareModule {
    patients: Vec<String>,
    diagnoses: Vec<String>,
}

impl HealthcareModule {
    pub fn new() -> Self {
        HealthcareModule {
            patients: Vec::new(),
            diagnoses: Vec::new(),
        }
    }

    pub fn add_patient(&mut self, name: &str) {
        self.patients.push(String::from(name));
    }

    pub fn remove_patient(&mut self, name: &str) -> bool {
        if let Some(index) = self.patients.iter().position(|p| p == name) {
            self.patients.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_diagnosis(&mut self, diagnosis: &str) {
        self.diagnoses.push(String::from(diagnosis));
    }

    pub fn get_patients(&self) -> Vec<String> {
        self.patients.clone()
    }

    pub fn get_diagnoses(&self) -> Vec<String> {
        self.diagnoses.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthcare_module() {
        let mut module = HealthcareModule::new();
        assert_eq!(module.get_patients().len(), 0);
        assert_eq!(module.get_diagnoses().len(), 0);

        module.add_patient("Alice");
        module.add_patient("Bob");
        assert_eq!(module.get_patients().len(), 2);
        assert!(module.remove_patient("Alice"));
        assert_eq!(module.get_patients().len(), 1);

        module.add_diagnosis("Flu");
        module.add_diagnosis("Cold");
        assert_eq!(module.get_diagnoses().len(), 2);
    }
}
