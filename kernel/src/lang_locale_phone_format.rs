extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct LangLocalePhoneFormat {
    locale: String,
    phone_format: String,
}

impl LangLocalePhoneFormat {
    pub fn new(locale: &str, phone_format: &str) -> Self {
        LangLocalePhoneFormat {
            locale: String::from(locale),
            phone_format: String::from(phone_format),
        }
    }

    pub fn get_locale(&self) -> &str {
        &self.locale
    }

    pub fn set_locale(&mut self, locale: &str) {
        self.locale = String::from(locale);
    }

    pub fn get_phone_format(&self) -> &str {
        &self.phone_format
    }

    pub fn set_phone_format(&mut self, phone_format: &str) {
        self.phone_format = String::from(phone_format);
    }

    pub fn format_phone_number(&self, number: &str) -> String {
        let mut formatted_number = String::new();
        let mut num_chars = 0;

        for c in number.chars() {
            if c.is_digit(10) {
                if num_chars > 0 && num_chars % self.phone_format.len() == 0 {
                    formatted_number.push('-');
                }
                formatted_number.push(c);
                num_chars += 1;
            }
        }

        formatted_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let formatter = LangLocalePhoneFormat::new("en-US", "(XXX) XXX-XXXX");
        assert_eq!(formatter.get_locale(), "en-US");
        assert_eq!(formatter.get_phone_format(), "(XXX) XXX-XXXX");
    }

    #[test]
    fn test_set_get_locale() {
        let mut formatter = LangLocalePhoneFormat::new("en-US", "(XXX) XXX-XXXX");
        formatter.set_locale("fr-FR");
        assert_eq!(formatter.get_locale(), "fr-FR");
    }

    #[test]
    fn test_set_get_phone_format() {
        let mut formatter = LangLocalePhoneFormat::new("en-US", "(XXX) XXX-XXXX");
        formatter.set_phone_format("XXX.XXX.XXXX");
        assert_eq!(formatter.get_phone_format(), "XXX.XXX.XXXX");
    }

    #[test]
    fn test_format_phone_number() {
        let formatter = LangLocalePhoneFormat::new("en-US", "(XXX) XXX-XXXX");
        let formatted = formatter.format_phone_number("1234567890");
        assert_eq!(formatted, "(123) 456-7890");
    }

    #[test]
    fn test_format_phone_number_with_non_digits() {
        let formatter = LangLocalePhoneFormat::new("en-US", "(XXX) XXX-XXXX");
        let formatted = formatter.format_phone_number("123-456-7890");
        assert_eq!(formatted, "(123) 456-7890");
    }
}
