extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangLocaleAddressFormat {
    language: String,
    locale: String,
    address_format: Vec<String>,
}

impl LangLocaleAddressFormat {
    pub fn new(language: &str, locale: &str) -> Self {
        let address_format = match (language, locale) {
            ("en", "US") => vec![
                String::from("Name"),
                String::from("Street Address"),
                String::from("City, State ZIP Code"),
            ],
            ("de", "DE") => vec![
                String::from("Name"),
                String::from("Straße und Hausnummer"),
                String::from("PLZ Ort"),
            ],
            _ => vec![],
        };
        LangLocaleAddressFormat {
            language: String::from(language),
            locale: String::from(locale),
            address_format,
        }
    }

    pub fn get_language(&self) -> &str {
        &self.language
    }

    pub fn get_locale(&self) -> &str {
        &self.locale
    }

    pub fn get_address_format(&self) -> &[String] {
        &self.address_format
    }

    pub fn set_language(&mut self, language: &str) {
        self.language = String::from(language);
    }

    pub fn set_locale(&mut self, locale: &str) {
        self.locale = String::from(locale);
        self.update_address_format();
    }

    fn update_address_format(&mut self) {
        self.address_format = match (&self.language[..], &self.locale[..]) {
            ("en", "US") => vec![
                String::from("Name"),
                String::from("Street Address"),
                String::from("City, State ZIP Code"),
            ],
            ("de", "DE") => vec![
                String::from("Name"),
                String::from("Straße und Hausnummer"),
                String::from("PLZ Ort"),
            ],
            _ => vec![],
        };
    }
}
