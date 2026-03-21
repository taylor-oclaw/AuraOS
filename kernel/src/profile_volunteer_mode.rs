extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn profile_caregiver_mode_init() {
    // Initialization logic for the module
}

pub extern "C" fn profile_caregiver_mode_exit() {
    // Cleanup logic for the module
}

pub struct CaregiverProfile {
    name: String,
    age: u32,
    patients: Vec<String>,
    experience_years: u32,
    certifications: Vec<String>,
}

impl CaregiverProfile {
    pub fn new(name: &str, age: u32) -> Self {
        CaregiverProfile {
            name: String::from(name),
            age,
            patients: Vec::new(),
            experience_years: 0,
            certifications: Vec::new(),
        }
    }

    pub fn add_patient(&mut self, patient_name: &str) {
        self.patients.push(String::from(patient_name));
    }

    pub fn remove_patient(&mut self, patient_name: &str) -> bool {
        if let Some(index) = self.patients.iter().position(|p| p == patient_name) {
            self.patients.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_certification(&mut self, certification: &str) {
        self.certifications.push(String::from(certification));
    }

    pub fn update_experience_years(&mut self, years: u32) {
        self.experience_years = years;
    }

    pub fn get_profile_info(&self) -> String {
        let mut info = String::from("info");
        info.push_str("Certifications:\n");
        for cert in &self.certifications {
            info.push_str(String::from("info").as_str());
        }
        info.push_str("Patients:\n");
        for patient in &self.patients {
            info.push_str(String::from("info").as_str());
        }
        info
    }
}
