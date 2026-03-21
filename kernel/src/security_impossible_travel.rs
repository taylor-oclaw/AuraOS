extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn security_impossible_travel_init() {
    // Initialization logic for the module
}

pub extern "C" fn security_impossible_travel_exit() {
    // Cleanup logic for the module
}

pub struct SecurityImpossibleTravel {
    allowed_countries: Vec<String>,
    blocked_countries: Vec<String>,
    travel_history: Vec<(String, String)>, // (from_country, to_country)
}

impl SecurityImpossibleTravel {
    pub fn new(allowed: Vec<String>, blocked: Vec<String>) -> Self {
        SecurityImpossibleTravel {
            allowed_countries: allowed,
            blocked_countries: blocked,
            travel_history: Vec::new(),
        }
    }

    pub fn add_allowed_country(&mut self, country: String) {
        if !self.allowed_countries.contains(&country) {
            self.allowed_countries.push(country);
        }
    }

    pub fn remove_allowed_country(&mut self, country: &str) -> bool {
        let index = self.allowed_countries.iter().position(|c| c == country);
        if let Some(i) = index {
            self.allowed_countries.remove(i);
            true
        } else {
            false
        }
    }

    pub fn add_blocked_country(&mut self, country: String) {
        if !self.blocked_countries.contains(&country) {
            self.blocked_countries.push(country);
        }
    }

    pub fn remove_blocked_country(&mut self, country: &str) -> bool {
        let index = self.blocked_countries.iter().position(|c| c == country);
        if let Some(i) = index {
            self.blocked_countries.remove(i);
            true
        } else {
            false
        }
    }

    pub fn is_travel_allowed(&self, from: &str, to: &str) -> bool {
        if self.blocked_countries.contains(&to.to_string()) {
            return false;
        }
        if self.allowed_countries.is_empty() || self.allowed_countries.contains(&to.to_string()) {
            true
        } else {
            false
        }
    }

    pub fn record_travel(&mut self, from: String, to: String) -> bool {
        if self.is_travel_allowed(&from, &to) {
            self.travel_history.push((from, to));
            true
        } else {
            false
        }
    }

    pub fn get_travel_history(&self) -> Vec<(String, String)> {
        self.travel_history.clone()
    }
}
