extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct MedicalDataDetector {
    data: Vec<String>,
    detected_anomalies: Vec<String>,
}

impl MedicalDataDetector {
    pub fn new() -> Self {
        MedicalDataDetector {
            data: Vec::new(),
            detected_anomalies: Vec::new(),
        }
    }

    pub fn add_data(&mut self, record: String) {
        self.data.push(record);
    }

    pub fn detect_anomalies(&mut self) {
        // Simple anomaly detection logic for demonstration
        for record in &self.data {
            if record.contains("anomaly") {
                self.detected_anomalies.push(record.clone());
            }
        }
    }

    pub fn get_detected_anomalies(&self) -> Vec<String> {
        self.detected_anomalies.clone()
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
        self.detected_anomalies.clear();
    }

    pub fn data_count(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_medical_data_detector() {
        let mut detector = MedicalDataDetector::new();
        assert_eq!(detector.data_count(), 0);

        detector.add_data(String::from("normal data"));
        detector.add_data(String::from("anomaly detected"));
        assert_eq!(detector.data_count(), 2);

        detector.detect_anomalies();
        let anomalies = detector.get_detected_anomalies();
        assert_eq!(anomalies.len(), 1);
        assert_eq!(anomalies[0], "anomaly detected");

        detector.clear_data();
        assert_eq!(detector.data_count(), 0);
    }
}
