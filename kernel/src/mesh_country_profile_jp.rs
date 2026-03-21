extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_country_profile_jp_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_country_profile_jp_exit() {
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

pub extern "C" fn mesh_country_profile_jp_create() -> *const CountryProfile {
    let profile = CountryProfile::new(
        "Japan",
        126_476_703,
        "Tokyo",
        &["Japanese"],
        "Yen (¥)",
    ;
    Box::leak(Box::new(profile))
}

pub extern "C" fn mesh_country_profile_jp_get_name(profile: *const CountryProfile) -> *const u8 {
    unsafe { (*profile).get_name().as_ptr() }
}

pub extern "C" fn mesh_country_profile_jp_get_population(profile: *const CountryProfile) -> u64 {
    unsafe { (*profile).get_population() }
}

pub extern "C" fn mesh_country_profile_jp_get_capital(profile: *const CountryProfile) -> *const u8 {
    unsafe { (*profile).get_capital().as_ptr() }
}

pub extern "C" fn mesh_country_profile_jp_get_languages_len(profile: *const CountryProfile) -> usize {
    unsafe { (*profile).get_languages().len() }
}

pub extern "C" fn mesh_country_profile_jp_get_language(profile: *const CountryProfile, index: usize) -> *const u8 {
    if let Some(language) = unsafe { (*profile).get_languages().get(index) } {
        language.as_ptr()
    } else {
        core::ptr::null()
    }
}

pub extern "C" fn mesh_country_profile_jp_get_currency(profile: *const CountryProfile) -> *const u8 {
    unsafe { (*profile).get_currency().as_ptr() }
}
