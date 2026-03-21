extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechRateDisorder {
    patient_id: String,
    diagnosis: String,
    symptoms: Vec<String>,
    treatment_plan: Vec<String>,
    progress_notes: Vec<String>,
}

impl SpeechRateDisorder {
    pub fn new(patient_id: &str, diagnosis: &str) -> Self {
        SpeechRateDisorder {
            patient_id: String::from(patient_id),
            diagnosis: String::from(diagnosis),
            symptoms: Vec::new(),
            treatment_plan: Vec::new(),
            progress_notes: Vec::new(),
        }
    }

    pub fn add_symptom(&mut self, symptom: &str) {
        self.symptoms.push(String::from(symptom));
    }

    pub fn get_symptoms(&self) -> &[String] {
        &self.symptoms
    }

    pub fn add_treatment_plan(&mut self, treatment: &str) {
        self.treatment_plan.push(String::from(treatment));
    }

    pub fn get_treatment_plan(&self) -> &[String] {
        &self.treatment_plan
    }

    pub fn add_progress_note(&mut self, note: &str) {
        self.progress_notes.push(String::from(note));
    }

    pub fn get_progress_notes(&self) -> &[String] {
        &self.progress_notes
    }
}
