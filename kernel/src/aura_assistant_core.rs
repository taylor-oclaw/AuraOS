extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_example() {
    // This is a placeholder for an FFI function that could be used to interact with the kernel.
}

pub struct AuraAssistantCore {
    knowledge_base: Vec<String>,
    user_queries: Vec<String>,
}

impl AuraAssistantCore {
    pub fn new() -> Self {
        AuraAssistantCore {
            knowledge_base: Vec::new(),
            user_queries: Vec::new(),
        }
    }

    pub fn add_to_knowledge_base(&mut self, fact: &str) {
        self.knowledge_base.push(String::from(fact));
    }

    pub fn get_knowledge_base_size(&self) -> usize {
        self.knowledge_base.len()
    }

    pub fn query_knowledge_base(&self, query: &str) -> Option<&String> {
        self.knowledge_base.iter().find(|&fact| fact.contains(query))
    }

    pub fn record_user_query(&mut self, query: &str) {
        self.user_queries.push(String::from(query));
    }

    pub fn get_user_query_history(&self) -> &[String] {
        &self.user_queries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_assistant_core() {
        let mut assistant = AuraAssistantCore::new();
        assert_eq!(assistant.get_knowledge_base_size(), 0);

        assistant.add_to_knowledge_base("Rust is a systems programming language.");
        assert_eq!(assistant.get_knowledge_base_size(), 1);

        let query_result = assistant.query_knowledge_base("Rust");
        assert!(query_result.is_some());
        assert_eq!(query_result.unwrap(), "Rust is a systems programming language.");

        assistant.record_user_query("What is Rust?");
        assert_eq!(assistant.get_user_query_history().len(), 1);
        assert_eq!(assistant.get_user_query_history()[0], "What is Rust?");
    }
}
