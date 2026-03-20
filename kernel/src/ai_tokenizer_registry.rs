extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AiTokenizerRegistry {
    tokenizers: Vec<String>,
}

impl AiTokenizerRegistry {
    pub fn new() -> Self {
        AiTokenizerRegistry {
            tokenizers: Vec::new(),
        }
    }

    pub fn register_tokenizer(&mut self, tokenizer_name: &str) {
        self.tokenizers.push(String::from(tokenizer_name));
    }

    pub fn unregister_tokenizer(&mut self, tokenizer_name: &str) -> bool {
        if let Some(index) = self.tokenizers.iter().position(|t| t == tokenizer_name) {
            self.tokenizers.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_tokenizers(&self) -> Vec<String> {
        self.tokenizers.clone()
    }

    pub fn has_tokenizer(&self, tokenizer_name: &str) -> bool {
        self.tokenizers.contains(&String::from(tokenizer_name))
    }

    pub fn count_tokenizers(&self) -> usize {
        self.tokenizers.len()
    }
}
