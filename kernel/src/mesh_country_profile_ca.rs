extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn mesh_country_profile_ca_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_country_profile_ca_exit() {
    // Cleanup logic for the module
}

pub struct CountryProfile {
    name: String,
    population: u64,
    capital: String,
    languages: Vec<String>,
    currency: String,
}

impl CountryProfile {
    pub fn new(name: &str, population: u64, capital: &str, languages: &[&str], currency: &str) -> Self {
        CountryProfile {
            name: String::from(name),
            population,
            capital: String::from(capital),
            languages: languages.iter().map(|&lang| String::from(lang)).collect(),
            currency: String::from(currency),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_population(&self) -> u64 {
        self.population
    }

    pub fn get_capital(&self) -> &str {
        &self.capital
    }

    pub fn get_languages(&self) -> &[String] {
        &self.languages
    }

    pub fn get_currency(&self) -> &str {
        &self.currency
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_country_profile() {
        let languages = vec!["English", "French"];
        let profile = CountryProfile::new("Canada", 37_742_154, "Ottawa", &languages, "Canadian Dollar");

        assert_eq!(profile.get_name(), "Canada");
        assert_eq!(profile.get_population(), 37_742_154);
        assert_eq!(profile.get_capital(), "Ottawa");
        assert_eq!(profile.get_languages(), &vec![String::from("English"), String::from("French")]);
        assert_eq!(profile.get_currency(), "Canadian Dollar");
    }
}
