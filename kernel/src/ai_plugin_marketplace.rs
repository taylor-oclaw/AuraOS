extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let marketplace = AIPluginMarketplace::new();
    marketplace.register_plugin("plugin1".into(), "description1".into());
    marketplace.register_plugin("plugin2".into(), "description2".into());

    loop {}
}

pub struct AIPluginMarketplace {
    plugins: Vec<(String, String)>,
}

impl AIPluginMarketplace {
    pub fn new() -> Self {
        AIPluginMarketplace {
            plugins: Vec::new(),
        }
    }

    pub fn register_plugin(&mut self, name: String, description: String) {
        self.plugins.push((name, description));
    }

    pub fn get_plugins(&self) -> &Vec<(String, String)> {
        &self.plugins
    }

    pub fn find_plugin_by_name(&self, name: &str) -> Option<&(String, String)> {
        self.plugins.iter().find(|&&(ref plugin_name, _)| plugin_name == name)
    }

    pub fn remove_plugin(&mut self, name: &str) -> bool {
        let pos = self.plugins.iter().position(|&(ref plugin_name, _)| plugin_name == name);
        if let Some(index) = pos {
            self.plugins.remove(index);
            true
        } else {
            false
        }
    }

    pub fn update_plugin_description(&mut self, name: &str, new_description: String) -> bool {
        if let Some(plugin) = self.plugins.iter_mut().find(|&&(ref plugin_name, _)| plugin_name == name) {
            plugin.1 = new_description;
            true
        } else {
            false
        }
    }
}
