extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AIProviderHotSwap {
    models: Vec<String>,
    current_model: usize,
}

impl AIProviderHotSwap {
    pub fn new() -> Self {
        AIProviderHotSwap {
            models: Vec::new(),
            current_model: 0,
        }
    }

    pub fn add_model(&mut self, model_name: &str) {
        self.models.push(String::from(model_name));
    }

    pub fn remove_model(&mut self, model_name: &str) -> bool {
        if let Some(index) = self.models.iter().position(|m| m == model_name) {
            self.models.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_models(&self) -> Vec<String> {
        self.models.clone()
    }

    pub fn switch_to_next_model(&mut self) {
        if !self.models.is_empty() {
            self.current_model = (self.current_model + 1) % self.models.len();
        }
    }

    pub fn current_model_name(&self) -> Option<&String> {
        self.models.get(self.current_model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_provider_hot_swap() {
        let mut ai = AIProviderHotSwap::new();
        assert_eq!(ai.list_models(), Vec::<String>::new());

        ai.add_model("model1");
        ai.add_model("model2");
        assert_eq!(ai.list_models(), vec![String::from("model1"), String::from("model2")]);

        assert!(ai.remove_model("model1"));
        assert_eq!(ai.list_models(), vec![String::from("model2")]);
        assert!(!ai.remove_model("model3"));

        ai.switch_to_next_model();
        assert_eq!(ai.current_model_name(), Some(&String::from("model2")));

        ai.add_model("model1");
        ai.switch_to_next_model();
        assert_eq!(ai.current_model_name(), Some(&String::from("model1")));
    }
}
