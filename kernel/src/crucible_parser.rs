extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
#[derive(Clone)]
pub enum CrucibleToken {
    Intent(String),
    Constraint(String, String),
    Pipe,
    With(String),
    As(String),
    From(String),
    Where(String),
    SortBy(String),
    Limit(u64),
    Unknown(String)
}
pub struct CrucibleProgram {
    pub intent: String,
    pub constraints: Vec<(String, String)>,
    pub source: Option<String>,
    pub output_format: Option<String>,
    pub sort: Option<String>,
    pub limit: Option<u64>
}
pub struct CrucibleParser {
    pub tokens: Vec<CrucibleToken>
}
impl CrucibleParser {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }
    pub fn tokenize(&mut self, input: &str) -> Vec<CrucibleToken> {
        let mut tokens = Vec::new();
        let parts: Vec<&str> = input.split(' ').collect();
        let mut i = 0;
        while i < parts.len() {
            match parts[i].to_lowercase().as_str() {
                "intent:" | "show" | "find" | "get" | "create" => {
                    let mut intent_words = Vec::new();
                    i += 1;
                    while i < parts.len() && !matches!(parts[i].to_lowercase().as_str(), "where" | "from" | "as" | "with" | "sort" | "limit") {
                        intent_words.push(parts[i]);
                        i += 1;
                    }
                    tokens.push(CrucibleToken::Intent(intent_words.join(" ")));
                },
                "where" => {
                    i += 1;
                    if i + 1 < parts.len() {
                        let field = String::from(parts[i]);
                        i += 1;
                        let value = String::from(parts[i]);
                        i += 1;
                        tokens.push(CrucibleToken::Where(field + " " + &value));
                    }
                },
                "from" => {
                    i += 1;
                    if i < parts.len() {
                        tokens.push(CrucibleToken::From(String::from(parts[i])));
                        i += 1;
                    }
                },
                "as" => {
                    i += 1;
                    if i < parts.len() {
                        tokens.push(CrucibleToken::As(String::from(parts[i])));
                        i += 1;
                    }
                },
                "sort" => {
                    i += 1;
                    if i < parts.len() && parts[i].to_lowercase() == "by" {
                        i += 1;
                    }
                    if i < parts.len() {
                        tokens.push(CrucibleToken::SortBy(String::from(parts[i])));
                        i += 1;
                    }
                },
                "limit" => {
                    i += 1;
                    if i < parts.len() {
                        if let Ok(n) = parts[i].parse::<u64>() {
                            tokens.push(CrucibleToken::Limit(n));
                        }
                        i += 1;
                    }
                },
                _ => {
                    tokens.push(CrucibleToken::Unknown(String::from(parts[i])));
                    i += 1;
                }
            }
        }
        self.tokens = tokens.clone();
        tokens
    }
    pub fn parse(&self) -> CrucibleProgram {
        let mut prog = CrucibleProgram {
            intent: String::new(),
            constraints: Vec::new(),
            source: None,
            output_format: None,
            sort: None,
            limit: None
        };
        for token in &self.tokens {
            match token {
                CrucibleToken::Intent(s) => prog.intent = s.clone(),
                CrucibleToken::Where(s) => {
                    let parts: Vec<&str> = s.splitn(2, ' ').collect();
                    if parts.len() == 2 {
                        prog.constraints.push((String::from(parts[0]), String::from(parts[1])));
                    }
                },
                CrucibleToken::From(s) => prog.source = Some(s.clone()),
                CrucibleToken::As(s) => prog.output_format = Some(s.clone()),
                CrucibleToken::SortBy(s) => prog.sort = Some(s.clone()),
                CrucibleToken::Limit(n) => prog.limit = Some(*n),
                _ => {}
            }
        }
        prog
    }
}
