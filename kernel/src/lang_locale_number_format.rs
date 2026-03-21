extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangLocaleNumberFormat {
    locale: String,
    number_format: String,
}

impl LangLocaleNumberFormat {
    pub fn new(locale: &str, number_format: &str) -> Self {
        LangLocaleNumberFormat {
            locale: String::from(locale),
            number_format: String::from(number_format),
        }
    }

    pub fn get_locale(&self) -> &String {
        &self.locale
    }

    pub fn set_locale(&mut self, locale: &str) {
        self.locale = String::from(locale);
    }

    pub fn get_number_format(&self) -> &String {
        &self.number_format
    }

    pub fn set_number_format(&mut self, number_format: &str) {
        self.number_format = String::from(number_format);
    }

    pub fn format_number(&self, number: u64) -> String {
        let mut formatted_number = number.to_string();
        if self.number_format.contains(",") {
            let mut parts: Vec<&str> = formatted_number.splitn(2, '.').collect();
            while parts[0].len() > 3 {
                parts.insert(1, &parts[0][parts[0].len() - 3..]);
                parts[0] = &parts[0][..parts[0].len() - 3];
            }
            formatted_number = parts.join(",");
        }
        if self.number_format.contains(".") {
            let mut parts: Vec<&str> = formatted_number.splitn(2, ',').collect();
            while parts[1].len() > 3 {
                parts.insert(2, &parts[1][..3]);
                parts[1] = &parts[1][3..];
            }
            formatted_number = parts.join(".");
        }
        formatted_number
    }
}
