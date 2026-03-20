extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut loader = ai_plugin_loader::AIPluginLoader::new();
    loader.load_plugin("plugin1");
    loader.load_plugin("plugin2");
    loader.unload_plugin("plugin1");
    loader.list_plugins();
    loader.get_plugin_status("plugin2");

    loop {}
}

mod ai_plugin_loader {
    use super::*;

    pub struct AIPluginLoader {
        plugins: Vec<String>,
        loaded_plugins: Vec<String>,
    }

    impl AIPluginLoader {
        pub fn new() -> Self {
            AIPluginLoader {
                plugins: Vec::new(),
                loaded_plugins: Vec::new(),
            }
        }

        pub fn load_plugin(&mut self, plugin_name: &str) {
            if !self.plugins.contains(&plugin_name.to_string()) {
                self.plugins.push(plugin_name.to_string());
            }
            if !self.loaded_plugins.contains(&plugin_name.to_string()) {
                self.loaded_plugins.push(plugin_name.to_string());
                println!("Loaded plugin: {}", plugin_name);
            } else {
                println!("Plugin already loaded: {}", plugin_name);
            }
        }

        pub fn unload_plugin(&mut self, plugin_name: &str) {
            if let Some(index) = self.loaded_plugins.iter().position(|x| x == plugin_name) {
                self.loaded_plugins.remove(index);
                println!("Unloaded plugin: {}", plugin_name);
            } else {
                println!("Plugin not found or already unloaded: {}", plugin_name);
            }
        }

        pub fn list_plugins(&self) {
            println!("Available plugins:");
            for plugin in &self.plugins {
                println!("{}", plugin);
            }
        }

        pub fn get_plugin_status(&self, plugin_name: &str) -> String {
            if self.loaded_plugins.contains(&plugin_name.to_string()) {
                format!("Plugin {} is loaded.", plugin_name)
            } else {
                format!("Plugin {} is not loaded.", plugin_name)
            }
        }
    }
}
