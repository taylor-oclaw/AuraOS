extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileDeviceUsagePatternLearn {
    device_id: String,
    usage_patterns: Vec<UsagePattern>,
}

impl ProfileDeviceUsagePatternLearn {
    pub fn new(device_id: &str) -> Self {
        ProfileDeviceUsagePatternLearn {
            device_id: String::from(device_id),
            usage_patterns: Vec::new(),
        }
    }

    pub fn add_usage_pattern(&mut self, pattern: UsagePattern) {
        self.usage_patterns.push(pattern);
    }

    pub fn get_device_id(&self) -> &str {
        &self.device_id
    }

    pub fn get_usage_patterns(&self) -> &[UsagePattern] {
        &self.usage_patterns
    }

    pub fn clear_usage_patterns(&mut self) {
        self.usage_patterns.clear();
    }

    pub fn find_pattern_by_type(&self, pattern_type: &str) -> Option<&UsagePattern> {
        self.usage_patterns.iter().find(|p| p.pattern_type == pattern_type)
    }
}

pub struct UsagePattern {
    pattern_type: String,
    frequency: u32,
    duration: u32,
}

impl UsagePattern {
    pub fn new(pattern_type: &str, frequency: u32, duration: u32) -> Self {
        UsagePattern {
            pattern_type: String::from(pattern_type),
            frequency,
            duration,
        }
    }

    pub fn get_pattern_type(&self) -> &str {
        &self.pattern_type
    }

    pub fn get_frequency(&self) -> u32 {
        self.frequency
    }

    pub fn get_duration(&self) -> u32 {
        self.duration
    }
}
