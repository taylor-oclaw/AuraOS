extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Translation {
    pub key: String,
    pub value: String,
}

pub struct Locale {
    pub code: String,
    pub name: String,
    pub rtl: bool,
    pub translations: Vec<Translation>,
}

pub struct I18nManager {
    pub locales: Vec<Locale>,
    pub active_locale: usize,
}

impl I18nManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            locales: Vec::new(),
            active_locale: 0,
        };

        let mut en = Locale {
            code: String::from("en"),
            name: String::from("English"),
            rtl: false,
            translations: Vec::new(),
        };
        en.translations.push(Translation {
            key: String::from("welcome"),
            value: String::from("Welcome to AuraOS"),
        });
        en.translations.push(Translation {
            key: String::from("shutdown"),
            value: String::from("Shut Down"),
        });
        en.translations.push(Translation {
            key: String::from("restart"),
            value: String::from("Restart"),
        });
        en.translations.push(Translation {
            key: String::from("settings"),
            value: String::from("Settings"),
        });
        en.translations.push(Translation {
            key: String::from("search"),
            value: String::from("Search"),
        });
        mgr.locales.push(en);

        let mut es = Locale {
            code: String::from("es"),
            name: String::from("Español"),
            rtl: false,
            translations: Vec::new(),
        };
        es.translations.push(Translation {
            key: String::from("welcome"),
            value: String::from("Bienvenido a AuraOS"),
        });
        es.translations.push(Translation {
            key: String::from("shutdown"),
            value: String::from("Apagar"),
        });
        es.translations.push(Translation {
            key: String::from("restart"),
            value: String::from("Reiniciar"),
        });
        es.translations.push(Translation {
            key: String::from("settings"),
            value: String::from("Configuración"),
        });
        es.translations.push(Translation {
            key: String::from("search"),
            value: String::from("Buscar"),
        });
        mgr.locales.push(es);

        mgr
    }

    pub fn t(&self, key: &str) -> &str {
        self.locales[self.active_locale]
            .translations
            .iter()
            .find(|t| t.key == key)
            .map(|t| t.value.as_str())
            .unwrap_or(key)
    }

    pub fn set_locale(&mut self, code: &str) {
        if let Some(idx) = self.locales.iter().position(|l| l.code == code) {
            self.active_locale = idx;
        }
    }

    pub fn is_rtl(&self) -> bool {
        self.locales[self.active_locale].rtl
    }

    pub fn available_locales(&self) -> Vec<&str> {
        self.locales.iter().map(|l| l.code.as_str()).collect()
    }
}
