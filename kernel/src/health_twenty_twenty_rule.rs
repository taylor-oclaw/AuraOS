extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct HealthTwentyTwentyRule {
    rules: Vec<String>,
}

impl HealthTwentyTwentyRule {
    pub fn new() -> Self {
        let mut rules = Vec::new();
        rules.push(String::from("Take a 20-second break every 20 minutes."));
        rules.push(String::from("Look at something 20 feet away for 20 seconds."));
        rules.push(String::from("Do some eye exercises to reduce strain."));
        rules.push(String::from("Drink water regularly to stay hydrated."));
        rules.push(String::from("Take a short walk or stretch during breaks."));

        HealthTwentyTwentyRule { rules }
    }

    pub fn get_all_rules(&self) -> &Vec<String> {
        &self.rules
    }

    pub fn add_rule(&mut self, rule: String) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, index: usize) -> Option<String> {
        if index < self.rules.len() {
            Some(self.rules.remove(index))
        } else {
            None
        }
    }

    pub fn get_rule(&self, index: usize) -> Option<&String> {
        self.rules.get(index)
    }

    pub fn count_rules(&self) -> usize {
        self.rules.len()
    }
}
