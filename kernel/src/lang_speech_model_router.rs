extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_speech_model_router_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_speech_model_router_exit() {
    // Cleanup logic for the module
}

pub struct LangSpeechModelRouter {
    models: Vec<String>,
    current_model: usize,
}

impl LangSpeechModelRouter {
    pub fn new(models: Vec<&str>) -> Self {
        let model_strings: Vec<String> = models.into_iter().map(String::from).collect();
        LangSpeechModelRouter {
            models: model_strings,
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

    pub fn switch_to_next_model(&mut self) {
        if !self.models.is_empty() {
            self.current_model = (self.current_model + 1) % self.models.len();
        }
    }

    pub fn get_current_model(&self) -> Option<&String> {
        self.models.get(self.current_model)
    }

    pub fn list_models(&self) -> Vec<&String> {
        self.models.iter().collect()
    }
}
