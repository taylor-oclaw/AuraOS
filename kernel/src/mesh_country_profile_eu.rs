extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() {}

struct MeshCountryProfileEU {
    country_name: String,
    population: u64,
    languages: Vec<String>,
    gdp_per_capita: f64,
    eu_member: bool,
}

impl MeshCountryProfileEU {
    pub fn new(country_name: &str, population: u64, languages: &[&str], gdp_per_capita: f64, eu_member: bool) -> Self {
        let lang_vec = languages.iter().map(|&lang| String::from(lang)).collect();
        MeshCountryProfileEU {
            country_name: String::from(country_name),
            population,
            languages: lang_vec,
            gdp_per_capita,
            eu_member,
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

    pub fn is_eu_member(&self) -> bool {
        self.eu_member
    }

    pub fn get_gdp_per_capita(&self) -> f64 {
        self.gdp_per_capita
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_country_profile_eu() {
        let mut profile = MeshCountryProfileEU::new("Germany", 83_000_000, &["German"], 49_000.0, true);
        assert_eq!(profile.get_country_name(), "Germany");
        assert_eq!(profile.population, 83_000_000);
        assert_eq!(profile.languages.len(), 1);
        assert_eq!(profile.is_eu_member(), true);
        assert_eq!(profile.get_gdp_per_capita(), 49_000.0);

        profile.set_population(84_000_000);
        assert_eq!(profile.population, 84_000_000);

        profile.add_language("English");
        assert_eq!(profile.languages.len(), 2);
    }
}
