extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_locale_measurement_init() {
    // Initialization code if needed
}

#[no_mangle]
pub extern "C" fn lang_locale_measurement_exit() {
    // Cleanup code if needed
}

pub struct LangLocaleMeasurement {
    language: String,
    locale: String,
    measurements: Vec<(String, u32)>,
}

impl LangLocaleMeasurement {
    pub fn new(language: &str, locale: &str) -> Self {
        LangLocaleMeasurement {
            language: String::from(language),
            locale: String::from(locale),
            measurements: Vec::new(),
        }
    }

    pub fn add_measurement(&mut self, name: &str, value: u32) {
        self.measurements.push((String::from(name), value));
    }

    pub fn get_language(&self) -> &str {
        &self.language
    }

    pub fn get_locale(&self) -> &str {
        &self.locale
    }

    pub fn get_measurements(&self) -> &Vec<(String, u32)> {
        &self.measurements
    }

    pub fn total_measurements(&self) -> u32 {
        self.measurements.iter().map(|&(_, value)| value).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_locale_measurement() {
        let mut measurement = LangLocaleMeasurement::new("English", "US");
        assert_eq!(measurement.get_language(), "English");
        assert_eq!(measurement.get_locale(), "US");

        measurement.add_measurement("Length", 10);
        measurement.add_measurement("Width", 20);

        assert_eq!(measurement.get_measurements().len(), 2);
        assert_eq!(measurement.total_measurements(), 30);
    }
}
