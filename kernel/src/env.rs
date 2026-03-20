extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

struct EnvVar {
    name: String,
    value: String,
}

struct Environment {
    vars: Vec<EnvVar>,
    hostname: String,
    username: String,
    home_dir: String,
    current_dir: String,
}

impl Environment {
    fn new() -> Self {
        Environment {
            vars: Vec::new(),
            hostname: String::from("aura"),
            username: String::from("user"),
            home_dir: String::from("/home/user"),
            current_dir: String::from("/home/user"),
        }
    }

    fn set_var(&mut self, name: &str, value: &str) {
        let var = EnvVar {
            name: String::from(name),
            value: String::from(value),
        };
        for v in &mut self.vars {
            if v.name == name {
                v.value = String::from(value);
                return;
            }
        }
        self.vars.push(var);
    }

    fn get_var(&self, name: &str) -> Option<&String> {
        for var in &self.vars {
            if var.name == name {
                return Some(&var.value);
            }
        }
        None
    }

    fn remove_var(&mut self, name: &str) {
        self.vars.retain(|v| v.name != name);
    }

    fn expand_vars(&self, input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '$' && chars.peek() == Some(&'{') {
                let mut var_name = String::new();
                chars.next(); // skip '{'
                while let Some(next_c) = chars.next() {
                    if next_c == '}' {
                        break;
                    }
                    var_name.push(next_c);
                }
                if let Some(value) = self.get_var(&var_name) {
                    result.push_str(value);
                } else {
                    result.push('$');
                    result.push('{');
                    result.push_str(&var_name);
                    result.push('}');
                }
            } else {
                result.push(c);
            }
        }

        result
    }

    fn hostname(&self) -> &String {
        &self.hostname
    }

    fn username(&self) -> &String {
        &self.username
    }

    fn home_dir(&self) -> &String {
        &self.home_dir
    }

    fn current_dir(&self) -> &String {
        &self.current_dir
    }

    fn set_current_dir(&mut self, dir: &str) {
        self.current_dir = String::from(dir);
    }

    fn prompt(&self) -> String {
        String::from("error")
    }
}
