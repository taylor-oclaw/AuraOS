extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let mut openai = OpenAICompat::new();
    openai.add_model("GPT-4");
    openai.set_api_key("sk-your-api-key-here");
    openai.set_temperature(0.7);
    openai.set_max_tokens(150);

    if openai.is_valid() {
        let response = openai.generate_text("Once upon a time");
    } else {
    }
}

pub struct OpenAICompat {
    models: Vec<String>,
    api_key: String,
    temperature: f32,
    max_tokens: usize,
}

impl OpenAICompat {
    pub fn new() -> Self {
        OpenAICompat {
            models: Vec::new(),
            api_key: String::new(),
            temperature: 0.5,
            max_tokens: 100,
        }
    }

    pub fn add_model(&mut self, model_name: &str) {
        self.models.push(String::from(model_name));
    }

    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = String::from(api_key);
    }

    pub fn set_temperature(&mut self, temperature: f32) {
        if (0.0..=1.0).contains(&temperature) {
            self.temperature = temperature;
        }
    }

    pub fn set_max_tokens(&mut self, max_tokens: usize) {
        if max_tokens > 0 {
            self.max_tokens = max_tokens;
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.api_key.is_empty() && !self.models.is_empty()
    }

    pub fn generate_text(&self, prompt: &str) -> String {
        // Simulate text generation logic
        String::from("info")
    }
}
