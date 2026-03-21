extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct LangLocaleMgr {
    locales: Vec<String>,
    current_locale: String,
}

impl LangLocaleMgr {
    pub fn new() -> Self {
        LangLocaleMgr {
            locales: Vec::new(),
            current_locale: String::from("en_US"),
        }
    }

    pub fn add_locale(&mut self, locale: &str) {
        if !self.locales.contains(&String::from(locale)) {
            self.locales.push(String::from(locale));
        }
    }

    pub fn remove_locale(&mut self, locale: &str) {
        self.locales.retain(|l| l != locale);
        if self.current_locale == locale {
            self.set_current_locale("en_US");
        }
    }

    pub fn set_current_locale(&mut self, locale: &str) {
        if self.locales.contains(&String::from(locale)) {
            self.current_locale = String::from(locale);
        } else {
            // Fallback to a default locale
            self.current_locale = String::from("en_US");
        }
    }

    pub fn get_current_locale(&self) -> &str {
        &self.current_locale
    }

    pub fn list_locales(&self) -> Vec<&str> {
        self.locales.iter().map(|l| l.as_str()).collect()
    }
}
