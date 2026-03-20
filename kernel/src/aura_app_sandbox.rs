extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_example() -> i32 {
    42
}

struct AuraAppSandbox {
    apps: Vec<String>,
    active_app_index: usize,
}

impl AuraAppSandbox {
    pub fn new() -> Self {
        AuraAppSandbox {
            apps: Vec::new(),
            active_app_index: 0,
        }
    }

    pub fn add_app(&mut self, app_name: &str) {
        self.apps.push(String::from(app_name));
    }

    pub fn remove_app(&mut self, app_name: &str) -> bool {
        if let Some(index) = self.apps.iter().position(|app| *app == app_name) {
            self.apps.remove(index);
            true
        } else {
            false
        }
    }

    pub fn activate_next_app(&mut self) {
        if !self.apps.is_empty() {
            self.active_app_index = (self.active_app_index + 1) % self.apps.len();
        }
    }

    pub fn get_active_app(&self) -> Option<&String> {
        if self.apps.is_empty() {
            None
        } else {
            Some(&self.apps[self.active_app_index])
        }
    }

    pub fn list_apps(&self) -> Vec<String> {
        self.apps.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_app_sandbox() {
        let mut sandbox = AuraAppSandbox::new();
        assert_eq!(sandbox.list_apps(), vec![]);

        sandbox.add_app("Calculator");
        sandbox.add_app("TextEditor");
        assert_eq!(sandbox.list_apps(), vec![String::from("Calculator"), String::from("TextEditor")]);

        assert!(sandbox.remove_app("Calculator"));
        assert_eq!(sandbox.list_apps(), vec![String::from("TextEditor")]);

        assert!(!sandbox.remove_app("NonExistentApp"));
        assert_eq!(sandbox.list_apps(), vec![String::from("TextEditor")]);

        sandbox.activate_next_app();
        assert_eq!(sandbox.get_active_app(), Some(&String::from("TextEditor")));

        sandbox.add_app("Browser");
        sandbox.activate_next_app();
        assert_eq!(sandbox.get_active_app(), Some(&String::from("Browser")));
    }
}
