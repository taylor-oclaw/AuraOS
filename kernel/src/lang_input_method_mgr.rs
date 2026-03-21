extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn lang_input_method_mgr_init() {
    // Initialization logic for the input method manager module
}

pub extern "C" fn lang_input_method_mgr_exit() {
    // Cleanup logic for the input method manager module
}

pub struct LangInputMethodMgr {
    methods: Vec<String>,
    current_method_index: usize,
}

impl LangInputMethodMgr {
    pub fn new(methods: Vec<&str>) -> Self {
        LangInputMethodMgr {
            methods: methods.into_iter().map(|m| m.to_string()).collect(),
            current_method_index: 0,
        }
    }

    pub fn add_method(&mut self, method: &str) {
        self.methods.push(method.to_string());
    }

    pub fn remove_method(&mut self, index: usize) -> Option<String> {
        if index < self.methods.len() {
            Some(self.methods.remove(index))
        } else {
            None
        }
    }

    pub fn switch_to_next_method(&mut self) {
        if !self.methods.is_empty() {
            self.current_method_index = (self.current_method_index + 1) % self.methods.len();
        }
    }

    pub fn get_current_method(&self) -> Option<&String> {
        self.methods.get(self.current_method_index)
    }

    pub fn list_methods(&self) -> Vec<&String> {
        self.methods.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_method_manager() {
        let mut manager = LangInputMethodMgr::new(vec!["English", "Spanish"]);
        assert_eq!(manager.get_current_method(), Some(&String::from("English")));

        manager.switch_to_next_method();
        assert_eq!(manager.get_current_method(), Some(&String::from("Spanish")));

        manager.add_method("French");
        assert_eq!(manager.list_methods(), vec![&String::from("English"), &String::from("Spanish"), &String::from("French")]);

        let removed = manager.remove_method(1);
        assert_eq!(removed, Some(String::from("Spanish")));
        assert_eq!(manager.list_methods(), vec![&String::from("English"), &String::from("French")]);
    }
}
