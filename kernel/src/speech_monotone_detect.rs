extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_monotone_detect_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_monotone_detect_exit() {
    // Cleanup logic for the module
}

pub struct SpeechMonotoneDetector {
    samples: Vec<i16>,
    threshold: i16,
}

impl SpeechMonotoneDetector {
    pub fn new(threshold: i16) -> Self {
        SpeechMonotoneDetector {
            samples: Vec::new(),
            threshold,
        }
    }

    pub fn add_sample(&mut self, sample: i16) {
        self.samples.push(sample);
    }

    pub fn is_monotonic(&self) -> bool {
        if self.samples.len() < 2 {
            return true;
        }

        let mut increasing = true;
        let mut decreasing = true;

        for window in self.samples.windows(2) {
            if window[0] < window[1] {
                decreasing = false;
            } else if window[0] > window[1] {
                increasing = false;
            }
        }

        increasing || decreasing
    }

    pub fn reset(&mut self) {
        self.samples.clear();
    }

    pub fn set_threshold(&mut self, threshold: i16) {
        self.threshold = threshold;
    }

    pub fn above_threshold(&self) -> bool {
        if let Some(&last_sample) = self.samples.last() {
            last_sample > self.threshold
        } else {
            false
        }
    }
}
