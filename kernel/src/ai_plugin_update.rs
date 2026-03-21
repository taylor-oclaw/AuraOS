extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

struct Plugin {
    name: String,
    version: String,
    dependencies: Vec<String>,
}

pub struct AiPluginUpdate {
    plugins: Vec<Plugin>,
}

impl AiPluginUpdate {
    pub fn new() -> Self {
        AiPluginUpdate { plugins: Vec::new() }
    }

    pub fn update_plugin(&mut self, name: &str, version: &str) {
        for plugin in self.plugins.iter_mut() {
            if plugin.name == name {
                plugin.version = String::from(version);
                return;
            }
        }
        self.plugins.push(Plugin {
            name: String::from(name),
            version: String::from(version),
            dependencies: Vec::new(),
        });
    }

    pub fn remove_plugin(&mut self, name: &str) {
        self.plugins.retain(|p| p.name != name);
    }

    pub fn add_dependency(&mut self, plugin_name: &str, dep: &str) {
        for plugin in self.plugins.iter_mut() {
            if plugin.name == plugin_name {
                plugin.dependencies.push(String::from(dep));
                return;
            }
        }
    }

    pub fn get_version(&self, name: &str) -> Option<String> {
        for plugin in &self.plugins {
            if plugin.name == name {
                return Some(plugin.version.clone());
            }
        }
        None
    }

    pub fn count(&self) -> usize {
        self.plugins.len()
    }
}
