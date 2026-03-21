extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct TravelVaccineCheck {
    vaccinated_countries: Vec<String>,
}

impl TravelVaccineCheck {
    pub fn new() -> Self {
        TravelVaccineCheck {
            vaccinated_countries: Vec::new(),
        }
    }

    pub fn add_vaccinated_country(&mut self, country: String) {
        if !self.vaccinated_countries.contains(&country) {
            self.vaccinated_countries.push(country);
        }
    }

    pub fn remove_vaccinated_country(&mut self, country: &str) {
        self.vaccinated_countries.retain(|c| c != country);
    }

    pub fn is_vaccinated(&self, country: &str) -> bool {
        self.vaccinated_countries.contains(country)
    }

    pub fn list_vaccinated_countries(&self) -> Vec<String> {
        self.vaccinated_countries.clone()
    }

    pub fn count_vaccinated_countries(&self) -> usize {
        self.vaccinated_countries.len()
    }
}
