extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct HealthWellnessReport {
    patient_name: String,
    age: u8,
    blood_pressure: (u16, u16), // (systolic, diastolic)
    heart_rate: u16,
    oxygen_saturation: u8,
}

impl HealthWellnessReport {
    pub fn new(patient_name: &str, age: u8, blood_pressure: (u16, u16), heart_rate: u16, oxygen_saturation: u8) -> Self {
        HealthWellnessReport {
            patient_name: String::from(patient_name),
            age,
            blood_pressure,
            heart_rate,
            oxygen_saturation,
        }
    }

    pub fn get_patient_name(&self) -> &str {
        &self.patient_name
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn get_blood_pressure(&self) -> (u16, u16) {
        self.blood_pressure
    }

    pub fn get_heart_rate(&self) -> u16 {
        self.heart_rate
    }

    pub fn get_oxygen_saturation(&self) -> u8 {
        self.oxygen_saturation
    }

    pub fn is_blood_pressure_normal(&self) -> bool {
        // Normal blood pressure range: 90-120/60-80 mmHg
        self.blood_pressure.0 >= 90 && self.blood_pressure.0 <= 120 &&
        self.blood_pressure.1 >= 60 && self.blood_pressure.1 <= 80
    }

    pub fn is_heart_rate_normal(&self) -> bool {
        // Normal heart rate range: 60-100 bpm for adults
        self.heart_rate >= 60 && self.heart_rate <= 100
    }

    pub fn is_oxygen_saturation_normal(&self) -> bool {
        // Normal oxygen saturation range: 95-100%
        self.oxygen_saturation >= 95 && self.oxygen_saturation <= 100
    }

    pub fn generate_report_summary(&self) -> String {
        let mut summary = String::from("Health Wellness Report:\n");
        summary.push_str(&format!("Patient Name: {}\n", self.patient_name));
        summary.push_str(&format!("Age: {}\n", self.age));
        summary.push_str(&format!("Blood Pressure: {}/{} mmHg\n", self.blood_pressure.0, self.blood_pressure.1));
        summary.push_str(&format!("Heart Rate: {} bpm\n", self.heart_rate));
        summary.push_str(&format!("Oxygen Saturation: {}%\n", self.oxygen_saturation));

        if self.is_blood_pressure_normal() {
            summary.push_str("Blood Pressure Status: Normal\n");
        } else {
            summary.push_str("Blood Pressure Status: Abnormal\n");
        }

        if self.is_heart_rate_normal() {
            summary.push_str("Heart Rate Status: Normal\n");
        } else {
            summary.push_str("Heart Rate Status: Abnormal\n");
        }

        if self.is_oxygen_saturation_normal() {
            summary.push_str("Oxygen Saturation Status: Normal\n");
        } else {
            summary.push_str("Oxygen Saturation Status: Abnormal\n");
        }

        summary
    }
}
