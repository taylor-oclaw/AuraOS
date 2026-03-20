extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let mut plugin = AiPluginUpdate::new();
    plugin.update_plugin("plugin1", "v2.0");
    plugin.add_dependency("plugin1", "libA");
    plugin.remove_dependency("plugin1", "libB");
    plugin.list_dependencies("plugin1");
    plugin.get_plugin_version("plugin1");
}

pub struct AiPluginUpdate {
    plugins: Vec<Plugin>,
}

impl AiPluginUpdate {
    pub fn new() -> Self {
        AiPluginUpdate { plugins: Vec::new() }
    }

    pub fn update_plugin(&mut self, name: &str, version: &str) {
        if let Some(plugin) = self.plugins.iter_mut().find(|p| p.name == name) {
            plugin.version = String::from(version);
        } else {
            self.plugins.push(Plugin {
                name: String::from(name),
                version: String::from(version),
                dependencies: Vec::new(),
            };
        }
    }

    pub fn add_dependency(&mut self, plugin_name: &str, dependency: &str) {
        if let Some(plugin) = self.plugins.iter_mut().find(|p| p.name == plugin_name) {
            if !plugin.dependencies.contains(&String::from(dependency)) {
                plugin.dependencies.push(String::from(dependency));
            }
        }
    }

    pub fn remove_dependency(&mut self, plugin_name: &str, dependency: &str) {
        if let Some(plugin) = self.plugins.iter_mut().find(|p| p.name == plugin_name) {
            plugin.dependencies.retain(|d| d != dependency);
        }
    }

    pub fn list_dependencies(&self, plugin_name: &str) {
        if let Some(plugin) = self.plugins.iter().find(|p| p.name == plugin_name) {
            for dep in &plugin.dependencies {
                // Simulate printing dependencies
            }
        }
    }

    pub fn get_plugin_version(&self, plugin_name: &str) -> Option<String> {
        self.plugins.iter()
            .find(|p| p.name == plugin_name)
            .map(|p| p.version.clone())
    }
}

struct Plugin {
    name: String,
    version: String,
    dependencies: Vec<String>,
}
