extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub struct ModelLoader {
    models: Vec<String>,
}

impl ModelLoader {
    pub fn new() -> Self {
        ModelLoader {
            models: Vec::new(),
        }
    }

    pub fn load_model(&mut self, model_name: &str) -> Result<(), &'static str> {
        if self.models.iter().any(|m| m == model_name) {
            Err("Model already loaded")
        } else {
            self.models.push(String::from(model_name));
            Ok(())
        }
    }

    pub fn unload_model(&mut self, model_name: &str) -> Result<(), &'static str> {
        match self.models.iter().position(|m| m == model_name) {
            Some(index) => {
                self.models.remove(index);
                Ok(())
            }
            None => Err("Model not found"),
        }
    }

    pub fn list_models(&self) -> Vec<String> {
        self.models.clone()
    }

    pub fn is_model_loaded(&self, model_name: &str) -> bool {
        self.models.iter().any(|m| m == model_name)
    }

    pub fn count_models(&self) -> usize {
        self.models.len()
    }
}
