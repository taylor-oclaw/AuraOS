extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn mesh_country_profile_br_init() {
    // Initialization code for the module
}

pub extern "C" fn mesh_country_profile_br_exit() {
    // Cleanup code for the module
}

pub struct CountryProfile {
    name: String,
    population: u64,
    languages: Vec<String>,
    capital: String,
    area_km2: f64,
}

impl CountryProfile {
    pub fn new(name: &str, population: u64, languages: &[&str], capital: &str, area_km2: f64) -> Self {
        CountryProfile {
            name: String::from(name),
            population,
            languages: languages.iter().map(|&lang| String::from(lang)).collect(),
            capital: String::from(capital),
            area_km2,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_population(&self) -> u64 {
        self.population
    }

    pub fn get_languages(&self) -> &[String] {
        &self.languages
    }

    pub fn get_capital(&self) -> &str {
        &self.capital
    }

    pub fn get_area_km2(&self) -> f64 {
        self.area_km2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_country_profile() {
        let languages = vec!["Portuguese", "Spanish"];
        let profile = CountryProfile::new("Brazil", 213_000_000, &languages, "Brasília", 8_515_767.0);

        assert_eq!(profile.get_name(), "Brazil");
        assert_eq!(profile.get_population(), 213_000_000);
        assert_eq!(profile.get_languages(), &vec![String::from("Portuguese"), String::from("Spanish")]);
        assert_eq!(profile.get_capital(), "Brasília");
        assert_eq!(profile.get_area_km2(), 8_515_767.0);
    }
}
