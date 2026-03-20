extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AIPluginManifest {
    name: String,
    version: String,
    description: String,
    dependencies: Vec<String>,
    capabilities: Vec<String>,
}

impl AIPluginManifest {
    pub fn new(name: &str, version: &str, description: &str) -> Self {
        AIPluginManifest {
            name: String::from(name),
            version: String::from(version),
            description: String::from(description),
            dependencies: Vec::new(),
            capabilities: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, dependency: &str) {
        self.dependencies.push(String::from(dependency));
    }

    pub fn remove_dependency(&mut self, dependency: &str) {
        self.dependencies.retain(|d| d != dependency);
    }

    pub fn add_capability(&mut self, capability: &str) {
        self.capabilities.push(String::from(capability));
    }

    pub fn remove_capability(&mut self, capability: &str) {
        self.capabilities.retain(|c| c != capability);
    }

    pub fn get_info(&self) -> String {
        let mut info = format!("Name: {}\nVersion: {}\nDescription: {}", self.name, self.version, self.description);
        if !self.dependencies.is_empty() {
            info.push_str("\nDependencies:\n");
            for dep in &self.dependencies {
                info.push_str(format!("- {}\n", dep).as_str());
            }
        }
        if !self.capabilities.is_empty() {
            info.push_str("Capabilities:\n");
            for cap in &self.capabilities {
                info.push_str(format!("- {}\n", cap).as_str());
            }
        }
        info
    }
}
