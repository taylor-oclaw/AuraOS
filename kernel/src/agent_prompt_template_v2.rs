extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentPromptTemplateV2 {
    template: String,
    variables: Vec<String>,
}

impl AgentPromptTemplateV2 {
    pub fn new(template: &str) -> Self {
        let mut variables = Vec::new();
        let mut current_var = String::new();
        let mut in_var = false;

        for c in template.chars() {
            if c == '{' {
                in_var = true;
            } else if c == '}' {
                if in_var && !current_var.is_empty() {
                    variables.push(current_var.clone());
                    current_var.clear();
                }
                in_var = false;
            } else if in_var {
                current_var.push(c);
            }
        }

        AgentPromptTemplateV2 {
            template: String::from(template),
            variables,
        }
    }

    pub fn get_template(&self) -> &str {
        &self.template
    }

    pub fn get_variables(&self) -> &[String] {
        &self.variables
    }

    pub fn set_variable(&mut self, index: usize, value: &str) {
        if index < self.variables.len() {
            self.variables[index] = String::from(value);
        }
    }

    pub fn render(&self) -> String {
        let mut rendered = String::new();
        let mut var_index = 0;

        for c in self.template.chars() {
            if c == '{' {
                if var_index < self.variables.len() {
                    rendered.push_str(&self.variables[var_index]);
                    var_index += 1;
                }
            } else if c == '}' {
                continue;
            } else {
                rendered.push(c);
            }
        }

        rendered
    }

    pub fn clear_variables(&mut self) {
        self.variables.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let template = "Hello, {name}! Today is {day}.";
        let agent = AgentPromptTemplateV2::new(template);
        assert_eq!(agent.get_template(), template);
        assert_eq!(agent.get_variables().len(), 2);
        assert_eq!(agent.get_variables()[0], "name");
        assert_eq!(agent.get_variables()[1], "day");
    }

    #[test]
    fn test_set_variable() {
        let mut agent = AgentPromptTemplateV2::new("Hello, {name}!");
        agent.set_variable(0, "Alice");
        assert_eq!(agent.get_variables()[0], "Alice");
    }

    #[test]
    fn test_render() {
        let mut agent = AgentPromptTemplateV2::new("Hello, {name}! Today is {day}.");
        agent.set_variable(0, "Alice");
        agent.set_variable(1, "Monday");
        assert_eq!(agent.render(), "Hello, Alice! Today is Monday.");
    }

    #[test]
    fn test_clear_variables() {
        let mut agent = AgentPromptTemplateV2::new("Hello, {name}!");
        agent.clear_variables();
        assert_eq!(agent.get_variables().len(), 0);
    }
}
