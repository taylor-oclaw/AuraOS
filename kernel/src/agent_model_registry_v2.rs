extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod agent_model_registry_v2 {
    use super::*;

    pub struct AgentModelRegistry {
        models: Vec<String>,
    }

    impl AgentModelRegistry {
        pub fn new() -> Self {
            AgentModelRegistry { models: Vec::new() }
        }

        pub fn register_model(&mut self, model_name: &str) {
            if !self.models.contains(&model_name.to_string()) {
                self.models.push(model_name.to_string());
            }
        }

        pub fn unregister_model(&mut self, model_name: &str) {
            self.models.retain(|m| m != model_name);
        }

        pub fn list_models(&self) -> Vec<String> {
            self.models.clone()
        }

        pub fn is_model_registered(&self, model_name: &str) -> bool {
            self.models.contains(&model_name.to_string())
        }

        pub fn count_models(&self) -> usize {
            self.models.len()
        }
    }
}
