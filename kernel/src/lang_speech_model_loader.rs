extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_speech_model_loader_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_speech_model_loader_exit() {
    // Cleanup logic for the module
}

pub struct LangSpeechModelLoader {
    models: Vec<String>,
    current_model: Option<usize>,
}

impl LangSpeechModelLoader {
    pub fn new() -> Self {
        LangSpeechModelLoader {
            models: Vec::new(),
            current_model: None,
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

    pub fn set_current_model(&mut self, model_name: &str) -> bool {
        if let Some(index) = self.models.iter().position(|m| m == model_name) {
            self.current_model = Some(index);
            true
        } else {
            false
        }
    }

    pub fn get_current_model(&self) -> Option<&String> {
        self.current_model.map(|index| &self.models[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_remove_model() {
        let mut loader = LangSpeechModelLoader::new();
        assert!(loader.add_model("model1"));
        assert_eq!(loader.list_models(), vec![String::from("model1")]);
        assert!(loader.remove_model("model1"));
        assert_eq!(loader.list_models(), Vec::<String>::new());
    }

    #[test]
    fn test_set_and_get_current_model() {
        let mut loader = LangSpeechModelLoader::new();
        loader.add_model("model1");
        loader.add_model("model2");
        assert!(loader.set_current_model("model1"));
        assert_eq!(loader.get_current_model(), Some(&String::from("model1")));
        assert!(loader.set_current_model("model2"));
        assert_eq!(loader.get_current_model(), Some(&String::from("model2")));
    }
}
