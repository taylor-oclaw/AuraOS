extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshCountryProfile {
    name: String,
    population: u64,
    area_km2: f64,
    languages: Vec<String>,
    capital: String,
}

impl MeshCountryProfile {
    pub fn new(name: &str, population: u64, area_km2: f64, languages: &[&str], capital: &str) -> Self {
        MeshCountryProfile {
            name: String::from(name),
            population,
            area_km2,
            languages: languages.iter().map(|&lang| String::from(lang)).collect(),
            capital: String::from(capital),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_population(&self) -> u64 {
        self.population
    }

    pub fn get_area_km2(&self) -> f64 {
        self.area_km2
    }

    pub fn get_languages(&self) -> &[String] {
        &self.languages
    }

    pub fn get_capital(&self) -> &str {
        &self.capital
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_country_profile() {
        let country = MeshCountryProfile::new(
            "Japan",
            126_476_703,
            377_975.0,
            &["Japanese"],
            "Tokyo",
        );

        assert_eq!(country.get_name(), "Japan");
        assert_eq!(country.get_population(), 126_476_703);
        assert_eq!(country.get_area_km2(), 377_975.0);
        assert_eq!(country.get_languages(), &["Japanese".to_string()]);
        assert_eq!(country.get_capital(), "Tokyo");
    }
}
