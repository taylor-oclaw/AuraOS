extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn runtime_ruby_init() {
    // Initialization logic for the module
}

pub extern "C" fn runtime_ruby_exit() {
    // Cleanup logic for the module
}

pub struct RubyRuntime {
    scripts: Vec<String>,
    variables: Vec<(String, String)>,
}

impl RubyRuntime {
    pub fn new() -> Self {
        RubyRuntime {
            scripts: Vec::new(),
            variables: Vec::new(),
        }
    }

    pub fn add_script(&mut self, script: &str) {
        self.scripts.push(String::from(script));
    }

    pub fn get_scripts(&self) -> &[String] {
        &self.scripts
    }

    pub fn set_variable(&mut self, name: &str, value: &str) {
        match self.variables.iter_mut().find(|v| v.0 == name) {
            Some(var) => var.1 = String::from(value),
            None => self.variables.push((String::from(name), String::from(value))),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.iter().find(|v| v.0 == name).map(|v| &v.1)
    }

    pub fn remove_variable(&mut self, name: &str) {
        self.variables.retain(|v| v.0 != name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ruby_runtime() {
        let mut runtime = RubyRuntime::new();
        assert!(runtime.get_scripts().is_empty());
        assert!(runtime.get_variable("test").is_none());

        runtime.add_script("puts 'Hello, World!'");
        assert_eq!(runtime.get_scripts().len(), 1);
        assert_eq!(runtime.get_scripts()[0], "puts 'Hello, World!'");

        runtime.set_variable("test", "value");
        assert_eq!(runtime.get_variable("test"), Some(&String::from("value")));

        runtime.set_variable("test", "new_value");
        assert_eq!(runtime.get_variable("test"), Some(&String::from("new_value")));

        runtime.remove_variable("test");
        assert!(runtime.get_variable("test").is_none());
    }
}
