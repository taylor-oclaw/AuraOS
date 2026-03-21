extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct PromptTemplate {
    pub name: String,
    pub template: String,
    pub variables: Vec<String>,
    pub category: String,
    pub usage_count: u64,
}

pub struct PromptManager {
    pub templates: Vec<PromptTemplate>,
}

impl PromptManager {
    pub fn new() -> Self {
        let templates = vec![
            PromptTemplate {
                name: String::from("summarize"),
                template: String::from("Summarize the following in 3 sentences: {content}"),
                variables: vec![String::from("content")],
                category: String::from("text"),
                usage_count: 0,
            },
            PromptTemplate {
                name: String::from("translate"),
                template: String::from("Translate to {language}: {text}"),
                variables: vec![String::from("language"), String::from("text")],
                category: String::from("text"),
                usage_count: 0,
            },
            PromptTemplate {
                name: String::from("code_review"),
                template: String::from("Review this code for bugs and improvements: {code}"),
                variables: vec![String::from("code")],
                category: String::from("dev"),
                usage_count: 0,
            },
            PromptTemplate {
                name: String::from("explain"),
                template: String::from("Explain {topic} in simple terms"),
                variables: vec![String::from("topic")],
                category: String::from("education"),
                usage_count: 0,
            },
            PromptTemplate {
                name: String::from("intent_parse"),
                template: String::from("Parse this user intent into structured command: {input}"),
                variables: vec![String::from("input")],
                category: String::from("system"),
                usage_count: 0,
            },
        ];

        Self { templates }
    }

    pub fn get(&self, name: &str) -> Option<&PromptTemplate> {
        self.templates.iter().find(|t| t.name == name)
    }

    pub fn render(&mut self, name: &str, vars: &[(String, String)]) -> Option<String> {
        if let Some(t) = self.templates.iter_mut().find(|t| t.name == name) {
            t.usage_count += 1;
            let mut result = t.template.clone();
            for (key, val) in vars {
                let placeholder = String::from("{") + key + "}";
                result = result.replace(placeholder.as_str(), val);
            }
            Some(result)
        } else {
            None
        }
    }

    pub fn by_category(&self, cat: &str) -> Vec<&PromptTemplate> {
        self.templates.iter().filter(|t| t.category == cat).collect()
    }

    pub fn add_template(&mut self, name: &str, template: &str, vars: Vec<String>, cat: &str) {
        self.templates.push(PromptTemplate {
            name: String::from(name),
            template: String::from(template),
            variables: vars,
            category: String::from(cat),
            usage_count: 0,
        });
    }
}
