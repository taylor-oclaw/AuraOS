extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn agent_test_generator_init() {
    // Initialization logic for the module
}

pub extern "C" fn agent_test_generator_exit() {
    // Cleanup logic for the module
}

pub struct AgentTestGenerator {
    tests: Vec<String>,
    current_index: usize,
}

impl AgentTestGenerator {
    pub fn new() -> Self {
        AgentTestGenerator {
            tests: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_test(&mut self, test_name: &str) {
        self.tests.push(String::from(test_name));
    }

    pub fn remove_test(&mut self, index: usize) -> Option<String> {
        if index < self.tests.len() {
            Some(self.tests.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_test(&self) -> Option<&String> {
        if self.current_index < self.tests.len() {
            Some(&self.tests[self.current_index])
        } else {
            None
        }
    }

    pub fn next_test(&mut self) -> Option<&String> {
        if self.current_index < self.tests.len() {
            let current = &self.tests[self.current_index];
            self.current_index += 1;
            Some(current)
        } else {
            None
        }
    }

    pub fn reset_tests(&mut self) {
        self.current_index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_test_generator() {
        let mut generator = AgentTestGenerator::new();
        assert_eq!(generator.get_current_test(), None);

        generator.add_test("Test1");
        generator.add_test("Test2");
        assert_eq!(generator.get_current_test(), Some(&String::from("Test1")));

        assert_eq!(generator.next_test(), Some(&String::from("Test1")));
        assert_eq!(generator.next_test(), Some(&String::from("Test2")));
        assert_eq!(generator.next_test(), None);

        generator.reset_tests();
        assert_eq!(generator.get_current_test(), Some(&String::from("Test1")));

        assert_eq!(generator.remove_test(0), Some(String::from("Test1")));
        assert_eq!(generator.get_current_test(), Some(&String::from("Test2")));
    }
}
