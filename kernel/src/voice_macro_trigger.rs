extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceMacroTrigger {
    macros: Vec<(String, String)>,
}

impl VoiceMacroTrigger {
    pub fn new() -> Self {
        VoiceMacroTrigger {
            macros: Vec::new(),
        }
    }

    pub fn add_macro(&mut self, trigger_word: &str, macro_text: &str) {
        let trigger = String::from(trigger_word);
        let macro_str = String::from(macro_text);
        self.macros.push((trigger, macro_str));
    }

    pub fn remove_macro(&mut self, trigger_word: &str) -> bool {
        let pos = self.macros.iter().position(|(trigger, _)| trigger == trigger_word);
        if let Some(index) = pos {
            self.macros.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_macro(&self, trigger_word: &str) -> Option<&String> {
        for (trigger, macro_text) in &self.macros {
            if trigger == trigger_word {
                return Some(macro_text);
            }
        }
        None
    }

    pub fn list_macros(&self) -> Vec<&String> {
        self.macros.iter().map(|(_, macro_text)| macro_text).collect()
    }

    pub fn clear_macros(&mut self) {
        self.macros.clear();
    }
}
