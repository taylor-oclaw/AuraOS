extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct AgentTokenCounter {
    tokens: Vec<String>,
}

impl AgentTokenCounter {
    pub fn new() -> Self {
        AgentTokenCounter {
            tokens: Vec::new(),
        }
    }

    pub fn add_token(&mut self, token: String) {
        self.tokens.push(token);
    }

    pub fn remove_token(&mut self, token: &str) -> bool {
        if let Some(index) = self.tokens.iter().position(|t| t == token) {
            self.tokens.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_token_count(&self) -> usize {
        self.tokens.len()
    }

    pub fn has_token(&self, token: &str) -> bool {
        self.tokens.contains(&String::from(token))
    }

    pub fn list_tokens(&self) -> Vec<String> {
        self.tokens.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_token_counter() {
        let mut counter = AgentTokenCounter::new();
        assert_eq!(counter.get_token_count(), 0);

        counter.add_token(String::from("token1"));
        assert_eq!(counter.get_token_count(), 1);
        assert!(counter.has_token("token1"));

        counter.add_token(String::from("token2"));
        assert_eq!(counter.get_token_count(), 2);
        assert!(counter.has_token("token2"));

        assert!(counter.remove_token("token1"));
        assert_eq!(counter.get_token_count(), 1);
        assert!(!counter.has_token("token1"));

        let tokens = counter.list_tokens();
        assert_eq!(tokens, vec![String::from("token2")]);

        assert!(!counter.remove_token("nonexistent"));
        assert_eq!(counter.get_token_count(), 1);
    }
}
