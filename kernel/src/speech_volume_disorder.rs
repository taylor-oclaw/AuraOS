extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_volume_disorder_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_volume_disorder_exit() {
    // Cleanup logic for the module
}

pub struct SpeechVolumeDisorder {
    patient_name: String,
    volume_levels: Vec<u8>,
    diagnosis: Option<String>,
}

impl SpeechVolumeDisorder {
    pub fn new(patient_name: &str) -> Self {
        SpeechVolumeDisorder {
            patient_name: String::from(patient_name),
            volume_levels: Vec::new(),
            diagnosis: None,
        }
    }

    pub fn add_volume_level(&mut self, level: u8) {
        self.volume_levels.push(level);
    }

    pub fn get_patient_name(&self) -> &str {
        &self.patient_name
    }

    pub fn analyze_volume(&mut self) {
        if self.volume_levels.is_empty() {
            return;
        }

        let average_level = self.volume_levels.iter().sum::<u8>() / self.volume_levels.len() as u8;

        if average_level < 50 {
            self.diagnosis = Some(String::from("Possible Hypoacusis"));
        } else if average_level > 100 {
            self.diagnosis = Some(String::from("Possible Hyperacusis"));
        } else {
            self.diagnosis = Some(String::from("Normal Volume Levels"));
        }
    }

    pub fn get_diagnosis(&self) -> Option<&str> {
        self.diagnosis.as_deref()
    }

    pub fn clear_volume_levels(&mut self) {
        self.volume_levels.clear();
    }
}
