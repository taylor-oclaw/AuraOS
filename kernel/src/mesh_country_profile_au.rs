extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod mesh_country_profile_au {
    use super::*;

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
}

#[cfg(test)]
mod tests {
    use super::mesh_country_profile_au::*;

    #[test]
    fn test_country_profile() {
        let profile = CountryProfile::new(
            "Australia",
            25_766_800,
            "Canberra",
            &["English", "Australian Aboriginal languages"],
            "Australian Dollar (AUD)",
        ;

        assert_eq!(profile.get_name(), "Australia");
        assert_eq!(profile.get_population(), 25_766_800);
        assert_eq!(profile.get_capital(), "Canberra");
        assert_eq!(
            profile.get_languages(),
            &[String::from("English"), String::from("Australian Aboriginal languages")]
        ;
        assert_eq!(profile.get_currency(), "Australian Dollar (AUD)");
    }
}
