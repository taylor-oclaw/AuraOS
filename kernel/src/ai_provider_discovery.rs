extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut provider = AiProviderDiscovery::new();
    provider.register_provider("provider1".into(), "AI Model 1".into());
    provider.register_provider("provider2".into(), "AI Model 2".into());

    if let Some(model) = provider.get_model_by_name("AI Model 1") {
    } else {
    }

    let providers = provider.list_providers();
    for provider in providers.iter() {
    }
}

pub struct AiProviderDiscovery {
    providers: Vec<(String, String)>,
}

impl AiProviderDiscovery {
    pub fn new() -> Self {
        AiProviderDiscovery {
            providers: Vec::new(),
        }
    }

    pub fn register_provider(&mut self, name: String, model: String) {
        self.providers.push((name, model));
    }

    pub fn get_model_by_name(&self, model_name: &str) -> Option<&String> {
        for (_, model) in self.providers.iter() {
            if model == model_name {
                return Some(model);
            }
        }
        None
    }

    pub fn list_providers(&self) -> Vec<String> {
        let mut provider_names = Vec::new();
        for (name, _) in self.providers.iter() {
            provider_names.push(name.clone());
        }
        provider_names
    }

    pub fn count_providers(&self) -> usize {
        self.providers.len()
    }
}
