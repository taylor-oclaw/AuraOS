extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshCountryProfileSG {
    country_name: String,
    population: u64,
    capital: String,
    languages: Vec<String>,
    currency: String,
}

impl MeshCountryProfileSG {
    pub fn new(country_name: &str, population: u64, capital: &str, languages: &[&str], currency: &str) -> Self {
        MeshCountryProfileSG {
            country_name: String::from(country_name),
            population,
            capital: String::from(capital),
            languages: languages.iter().map(|&lang| String::from(lang)).collect(),
            currency: String::from(currency),
        }
    }

    pub fn get_country_name(&self) -> &str {
        &self.country_name
    }

    pub fn set_population(&mut self, population: u64) {
        self.population = population;
    }

    pub fn add_language(&mut self, language: &str) {
        self.languages.push(String::from(language));
    }

    pub fn get_capital(&self) -> &str {
        &self.capital
    }

    pub fn get_currency(&self) -> &str {
        &self.currency
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_country_profile_sg() {
        let mut profile = MeshCountryProfileSG::new(
            "Singapore",
            5_704_200,
            "Singapore",
            &["English", "Malay", "Tamil"],
            "Singapore Dollar (SGD)",
        );

        assert_eq!(profile.get_country_name(), "Singapore");
        assert_eq!(profile.population, 5_704_200);
        assert_eq!(profile.get_capital(), "Singapore");
        assert_eq!(profile.get_currency(), "Singapore Dollar (SGD)");

        profile.set_population(6_000_000);
        assert_eq!(profile.population, 6_000_000);

        profile.add_language("Chinese");
        assert_eq!(profile.languages.len(), 4);
        assert!(profile.languages.contains(&String::from("Chinese")));
    }
}
