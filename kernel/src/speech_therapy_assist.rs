extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechTherapyAssist {
    patient_name: String,
    therapy_sessions: Vec<String>,
    progress_notes: Vec<String>,
}

impl SpeechTherapyAssist {
    pub fn new(patient_name: &str) -> Self {
        SpeechTherapyAssist {
            patient_name: String::from(patient_name),
            therapy_sessions: Vec::new(),
            progress_notes: Vec::new(),
        }
    }

    pub fn add_session(&mut self, session_details: &str) {
        self.therapy_sessions.push(String::from(session_details));
    }

    pub fn get_patient_name(&self) -> &str {
        &self.patient_name
    }

    pub fn list_sessions(&self) -> &[String] {
        &self.therapy_sessions
    }

    pub fn add_progress_note(&mut self, note: &str) {
        self.progress_notes.push(String::from(note));
    }

    pub fn get_progress_notes(&self) -> &[String] {
        &self.progress_notes
    }
}
