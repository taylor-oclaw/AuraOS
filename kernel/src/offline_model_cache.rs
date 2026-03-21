extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut cache = OfflineModelCache::new();
    cache.add_model("model1", b"model_data_1");
    cache.add_model("model2", b"model_data_2");

    if let Some(data) = cache.get_model("model1") {
        // Process model data
    }

    loop {}
}

pub struct OfflineModelCache {
    models: Vec<(String, Vec<u8>)>,
}

impl OfflineModelCache {
    pub fn new() -> Self {
        OfflineModelCache { models: Vec::new() }
    }

    pub fn add_model(&mut self, name: &str, data: &[u8]) {
        let model_name = String::from(name);
        let model_data = Vec::from(data);
        self.models.push((model_name, model_data));
    }

    pub fn get_model(&self, name: &str) -> Option<&Vec<u8>> {
        for (model_name, data) in &self.models {
            if model_name == name {
                return Some(data);
            }
        }
        None
    }

    pub fn remove_model(&mut self, name: &str) {
        self.models.retain(|(model_name, _)| model_name != name);
    }

    pub fn list_models(&self) -> Vec<&String> {
        self.models.iter().map(|(name, _)| name).collect()
    }
}
