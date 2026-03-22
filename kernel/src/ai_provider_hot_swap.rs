extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let provider = AiProviderHotSwap::new();
    provider.load_model("model1");
    provider.unload_model("model2");
    provider.list_models();
    provider.swap_models("model3", "model4");
    provider.get_current_model();
}

pub struct AiProviderHotSwap {
    models: Vec<String>,
    current_model: Option<String>,
}

impl AiProviderHotSwap {
    pub fn new() -> Self {
        AiProviderHotSwap {
            models: Vec::new(),
            current_model: None,
        }
    }

    pub fn load_model(&mut self, model_name: &str) {
        if !self.models.contains(&model_name.to_string()) {
            self.models.push(model_name.to_string());
            println!("Model {} loaded.", model_name);
        } else {
            println!("Model {} is already loaded.", model_name);
        }
    }

    pub fn unload_model(&mut self, model_name: &str) {
        if let Some(index) = self.models.iter().position(|m| m == model_name) {
            self.models.remove(index);
            if self.current_model.as_ref() == Some(&model_name.to_string()) {
                self.current_model = None;
            }
            println!("Model {} unloaded.", model_name);
        } else {
            println!("Model {} not found.", model_name);
        }
    }

    pub fn list_models(&self) {
        for model in &self.models {
            println!("{}", model);
        }
    }

    pub fn swap_models(&mut self, old_model: &str, new_model: &str) {
        if let Some(index) = self.models.iter().position(|m| m == old_model) {
            self.models[index] = new_model.to_string();
            if self.current_model.as_ref() == Some(&old_model.to_string()) {
                self.current_model = Some(new_model.to_string());
            }
            println!("Model swapped from {} to {}", old_model, new_model);
        } else {
            println!("Old model {} not found.", old_model);
        }
    }

    pub fn get_current_model(&self) -> Option<&str> {
        self.current_model.as_deref()
    }
}