extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

pub struct AuraAssistantProactive {
    knowledge_base: Vec<String>,
    user_queries: Vec<String>,
}

impl AuraAssistantProactive {
    pub fn new() -> Self {
        AuraAssistantProactive {
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

    pub fn query(&mut self, query: &str) -> Option<&String> {
        self.user_queries.push(String::from(query));
        // Simple linear search for demonstration purposes
        self.knowledge_base.iter().find(|&fact| fact.contains(query))
    }

    pub fn get_query_history_size(&self) -> usize {
        self.user_queries.len()
    }

    pub fn clear_knowledge_base(&mut self) {
        self.knowledge_base.clear();
    }
}
