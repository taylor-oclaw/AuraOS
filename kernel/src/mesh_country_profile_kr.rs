extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_country_profile_kr_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_country_profile_kr_exit() {
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
        let profile = CountryProfile::new(
            "South Korea",
            51_780_834,
            "Seoul",
            &["Korean"],
            "South Korean won (WON)",
        );

        assert_eq!(profile.get_name(), "South Korea");
        assert_eq!(profile.get_population(), 51_780_834);
        assert_eq!(profile.get_capital(), "Seoul");
        assert_eq!(profile.get_languages()[0], "Korean");
        assert_eq!(profile.get_currency(), "South Korean won (WON)");
    }
}
