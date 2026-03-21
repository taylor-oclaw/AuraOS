extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_country_profile_cn_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_country_profile_cn_exit() {
    // Cleanup logic for the module
}

pub struct CountryProfile {
    name: String,
    population: u64,
    capital: String,
    languages: Vec<String>,
    gdp_per_capita: f64,
}

impl CountryProfile {
    pub fn new(name: &str, population: u64, capital: &str, languages: &[&str], gdp_per_capita: f64) -> Self {
        CountryProfile {
            name: String::from(name),
            population,
            capital: String::from(capital),
            languages: languages.iter().map(|&lang| String::from(lang)).collect(),
            gdp_per_capita,
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

    pub fn get_gdp_per_capita(&self) -> f64 {
        self.gdp_per_capita
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_country_profile() {
        let profile = CountryProfile::new(
            "China",
            1_400_000_000,
            "Beijing",
            &["Mandarin", "Cantonese"],
            8_500.0,
        ;

        assert_eq!(profile.get_name(), "China");
        assert_eq!(profile.get_population(), 1_400_000_000);
        assert_eq!(profile.get_capital(), "Beijing");
        assert_eq!(profile.get_languages(), &[String::from("Mandarin"), String::from("Cantonese")]);
        assert_eq!(profile.get_gdp_per_capita(), 8_500.0);
    }
}
