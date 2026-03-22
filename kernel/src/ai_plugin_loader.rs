extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let loader = AIPluginLoader::new();
    loader.load_plugin("plugin1");
    loader.load_plugin("plugin2");
    loader.unload_plugin("plugin1");
    println!("Plugins loaded: {:?}", loader.list_plugins());
}

pub struct AIPluginLoader {
    plugins: Vec<String>,
}

impl AIPluginLoader {
    pub fn new() -> Self {
        AIPluginLoader {
            plugins: Vec::new(),
        }
    }

    pub fn load_plugin(&mut self, plugin_name: &str) {
        if !self.plugins.contains(&plugin_name.to_string()) {
            self.plugins.push(plugin_name.to_string());
            println!("Loaded plugin: {}", plugin_name);
        } else {
            println!("Plugin already loaded: {}", plugin_name);
        }
    }

    pub fn unload_plugin(&mut self, plugin_name: &str) {
        if let Some(index) = self.plugins.iter().position(|x| x == plugin_name) {
            self.plugins.remove(index);
            println!("Unloaded plugin: {}", plugin_name);
        } else {
            println!("Plugin not found: {}", plugin_name);
        }
    }

    pub fn list_plugins(&self) -> Vec<String> {
        self.plugins.clone()
    }

    pub fn is_plugin_loaded(&self, plugin_name: &str) -> bool {
        self.plugins.contains(&plugin_name.to_string())
    }

    pub fn count_plugins(&self) -> usize {
        self.plugins.len()
    }
}