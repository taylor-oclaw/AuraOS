extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct TooltipSystem {
    tooltips: Vec<(String, String)>,
}

impl TooltipSystem {
    pub fn new() -> Self {
        TooltipSystem {
            tooltips: Vec::new(),
        }
    }

    pub fn add_tooltip(&mut self, key: &str, value: &str) {
        let key_str = String::from(key);
        let value_str = String::from(value);
        self.tooltips.push((key_str, value_str));
    }

    pub fn get_tooltip(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.tooltips {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove_tooltip(&mut self, key: &str) {
        self.tooltips.retain(|(k, _)| k != key);
    }

    pub fn list_tooltips(&self) -> Vec<&String> {
        self.tooltips.iter().map(|(_, v)| v).collect()
    }

    pub fn clear_tooltips(&mut self) {
        self.tooltips.clear();
    }
}
