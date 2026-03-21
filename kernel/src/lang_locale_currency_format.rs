extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LocaleCurrencyFormatter {
    locale: String,
    currency_symbol: String,
}

impl LocaleCurrencyFormatter {
    pub fn new(locale: &str, currency_symbol: &str) -> Self {
        LocaleCurrencyFormatter {
            locale: String::from(locale),
            currency_symbol: String::from(currency_symbol),
        }
    }

    pub fn format_currency(&self, amount: u64) -> String {
        let mut formatted_amount = amount.to_string();
        if formatted_amount.len() > 3 {
            let mut result = Vec::new();
            for (i, c) in formatted_amount.chars().rev().enumerate() {
                if i % 3 == 0 && i != 0 {
                    result.push(',');
                }
                result.push(c);
            }
            formatted_amount = result.into_iter().rev().collect();
        }
        String::from("info")
    }

    pub fn get_locale(&self) -> &str {
        &self.locale
    }

    pub fn set_locale(&mut self, locale: &str) {
        self.locale = String::from(locale);
    }

    pub fn get_currency_symbol(&self) -> &str {
        &self.currency_symbol
    }

    pub fn set_currency_symbol(&mut self, currency_symbol: &str) {
        self.currency_symbol = String::from(currency_symbol);
    }
}
