extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TravelCurrencyConverter {
    exchange_rates: Vec<(String, String, f64)>,
}

impl TravelCurrencyConverter {
    pub fn new() -> Self {
        TravelCurrencyConverter {
            exchange_rates: Vec::new(),
        }
    }

    pub fn add_exchange_rate(&mut self, from_currency: &str, to_currency: &str, rate: f64) {
        self.exchange_rates.push((from_currency.to_string(), to_currency.to_string(), rate));
    }

    pub fn get_exchange_rate(&self, from_currency: &str, to_currency: &str) -> Option<f64> {
        for (f, t, r) in &self.exchange_rates {
            if f == from_currency && t == to_currency {
                return Some(*r);
            }
        }
        None
    }

    pub fn convert(&self, amount: f64, from_currency: &str, to_currency: &str) -> Option<f64> {
        match self.get_exchange_rate(from_currency, to_currency) {
            Some(rate) => Some(amount * rate),
            None => None,
        }
    }

    pub fn list_currencies(&self) -> Vec<String> {
        let mut currencies = Vec::new();
        for (from, to, _) in &self.exchange_rates {
            if !currencies.contains(from) {
                currencies.push(from.clone());
            }
            if !currencies.contains(to) {
                currencies.push(to.clone());
            }
        }
        currencies
    }

    pub fn remove_exchange_rate(&mut self, from_currency: &str, to_currency: &str) -> bool {
        let original_len = self.exchange_rates.len();
        self.exchange_rates.retain(|(f, t, _)| f != from_currency || t != to_currency);
        original_len != self.exchange_rates.len()
    }
}
