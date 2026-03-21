extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn health_vision_test_init() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn health_vision_test_exit() {
}

pub struct HealthVisionTest {
    name: String,
    tests_passed: usize,
    tests_failed: usize,
    test_results: Vec<String>,
}

impl HealthVisionTest {
    pub fn new(name: &str) -> Self {
        HealthVisionTest {
            name: String::from(name),
            tests_passed: 0,
            tests_failed: 0,
            test_results: Vec::new(),
        }
    }

    pub fn run_test(&mut self, test_name: &str, result: bool) {
        if result {
            self.tests_passed += 1;
            self.test_results.push(format!("{} passed", test_name));
        } else {
            self.tests_failed += 1;
            self.test_results.push(format!("{} failed", test_name));
        }
    }

    pub fn get_test_results(&self) -> &Vec<String> {
        &self.test_results
    }

    pub fn get_pass_rate(&self) -> f32 {
        if self.tests_passed + self.tests_failed == 0 {
            0.0
        } else {
            (self.tests_passed as f32 / (self.tests_passed + self.tests_failed) as f32) * 100.0
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
