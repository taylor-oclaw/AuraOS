extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct PerfUsagePatternLearn {
    patterns: Vec<String>,
}

impl PerfUsagePatternLearn {
    pub fn new() -> Self {
        PerfUsagePatternLearn {
            patterns: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: String) {
        self.patterns.push(pattern);
    }

    pub fn remove_pattern(&mut self, index: usize) -> Option<String> {
        if index < self.patterns.len() {
            Some(self.patterns.remove(index))
        } else {
            None
        }
    }

    pub fn get_pattern(&self, index: usize) -> Option<&String> {
        self.patterns.get(index)
    }

    pub fn list_patterns(&self) -> &[String] {
        &self.patterns
    }

    pub fn clear_patterns(&mut self) {
        self.patterns.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perf_usage_pattern_learn() {
        let mut perf = PerfUsagePatternLearn::new();
        assert_eq!(perf.list_patterns().len(), 0);

        perf.add_pattern(String::from("pattern1"));
        perf.add_pattern(String::from("pattern2"));
        assert_eq!(perf.list_patterns().len(), 2);
        assert_eq!(perf.get_pattern(0), Some(&String::from("pattern1")));
        assert_eq!(perf.get_pattern(1), Some(&String::from("pattern2")));

        let removed = perf.remove_pattern(0);
        assert_eq!(removed, Some(String::from("pattern1")));
        assert_eq!(perf.list_patterns().len(), 1);

        perf.clear_patterns();
        assert_eq!(perf.list_patterns().len(), 0);
    }
}
