extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct LangAccentFrenchFr {
    phrases: Vec<String>,
}

impl LangAccentFrenchFr {
    pub fn new() -> Self {
        let mut phrases = Vec::new();
        phrases.push(String::from("Bonjour, comment ça va?"));
        phrases.push(String::from("Je m'appelle [Name]."));
        phrases.push(String::from("J'aime les croissants."));
        phrases.push(String::from("C'est magnifique!"));
        phrases.push(String::from("Au revoir!"));

        LangAccentFrenchFr { phrases }
    }

    pub fn get_greeting(&self) -> &str {
        &self.phrases[0]
    }

    pub fn introduce_yourself(&self, name: &str) -> String {
        let mut introduction = self.phrases[1].clone();
        introduction.replace_range(9..13, name);
        introduction
    }

    pub fn get_food_preference(&self) -> &str {
        &self.phrases[2]
    }

    pub fn express_excitement(&self) -> &str {
        &self.phrases[3]
    }

    pub fn say_goodbye(&self) -> &str {
        &self.phrases[4]
    }
}
