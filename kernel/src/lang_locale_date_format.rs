extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangLocaleDateFormat {
    language: String,
    locale: String,
    date_format: String,
}

impl LangLocaleDateFormat {
    pub fn new(language: &str, locale: &str, date_format: &str) -> Self {
        LangLocaleDateFormat {
            language: String::from(language),
            locale: String::from(locale),
            date_format: String::from(date_format),
        }
    }

    pub fn get_language(&self) -> &str {
        &self.language
    }

    pub fn set_language(&mut self, language: &str) {
        self.language = String::from(language);
    }

    pub fn get_locale(&self) -> &str {
        &self.locale
    }

    pub fn set_locale(&mut self, locale: &str) {
        self.locale = String::from(locale);
    }

    pub fn get_date_format(&self) -> &str {
        &self.date_format
    }

    pub fn set_date_format(&mut self, date_format: &str) {
        self.date_format = String::from(date_format);
    }
}
