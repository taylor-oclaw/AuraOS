extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechAnxietyDetector {
    data: Vec<u8>,
    threshold: u8,
}

impl SpeechAnxietyDetector {
    pub fn new(threshold: u8) -> Self {
        SpeechAnxietyDetector {
            data: Vec::new(),
            threshold,
        }
    }

    pub fn add_sample(&mut self, sample: u8) {
        self.data.push(sample);
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn set_threshold(&mut self, threshold: u8) {
        self.threshold = threshold;
    }

    pub fn detect_anxiety(&self) -> bool {
        if self.data.is_empty() {
            return false;
        }
        let average: u8 = self.data.iter().sum::<u8>() / self.data.len() as u8;
        average > self.threshold
    }

    pub fn get_data_size(&self) -> usize {
        self.data.len()
    }
}
