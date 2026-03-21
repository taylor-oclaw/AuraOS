extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_country_profile_uk_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_country_profile_uk_exit() {
    // Cleanup logic for the module
}

pub struct UKProfile {
    name: String,
    population: u64,
    capital: String,
    languages: Vec<String>,
    currency: String,
}

impl UKProfile {
    pub fn new(name: &str, population: u64, capital: &str, languages: &[&str], currency: &str) -> Self {
        UKProfile {
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

#[no_mangle]
pub extern "C" fn create_uk_profile() -> *const UKProfile {
    let uk = UKProfile::new(
        "United Kingdom",
        67_881_900,
        "London",
        &["English", "Welsh", "Scottish Gaelic", "British Sign Language"],
        "Pound Sterling",
    );
    Box::leak(Box::new(uk))
}

#[no_mangle]
pub extern "C" fn get_uk_name(profile: *const UKProfile) -> *const u8 {
    unsafe { (*profile).get_name().as_ptr() }
}

#[no_mangle]
pub extern "C" fn get_uk_population(profile: *const UKProfile) -> u64 {
    unsafe { (*profile).get_population() }
}

#[no_mangle]
pub extern "C" fn get_uk_capital(profile: *const UKProfile) -> *const u8 {
    unsafe { (*profile).get_capital().as_ptr() }
}

#[no_mangle]
pub extern "C" fn get_uk_languages(profile: *const UKProfile, index: usize) -> *const u8 {
    if let Some(lang) = unsafe { (*profile).get_languages().get(index) } {
        lang.as_ptr()
    } else {
        core::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn get_uk_currency(profile: *const UKProfile) -> *const u8 {
    unsafe { (*profile).get_currency().as_ptr() }
}
