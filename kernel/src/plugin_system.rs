extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum PluginState {
    Loaded,
    Active,
    Disabled,
    Error(String),
}

pub struct Plugin {
    pub id: u64,
    pub name: String,
    pub version: String,
    pub author: String,
    pub state: PluginState,
    pub hooks: Vec<String>,
    pub priority: u8,
    pub memory_bytes: u64,
}

pub struct PluginSystem {
    pub plugins: Vec<Plugin>,
    pub next_id: u64,
    pub max_plugins: usize,
}

impl PluginSystem {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            next_id: 1,
            max_plugins: 100,
        }
    }

    pub fn load(&mut self, name: &str, version: &str, author: &str, hooks: Vec<String>) -> Option<u64> {
        if self.plugins.len() >= self.max_plugins {
            return None;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.plugins.push(Plugin {
            id,
            name: String::from(name),
            version: String::from(version),
            author: String::from(author),
            state: PluginState::Loaded,
            hooks,
            priority: 50,
            memory_bytes: 0,
        });
        Some(id)
    }

    pub fn activate(&mut self, id: u64) -> bool {
        if let Some(p) = self.plugins.iter_mut().find(|p| p.id == id) {
            p.state = PluginState::Active;
            true
        } else {
            false
        }
    }

    pub fn disable(&mut self, id: u64) {
        if let Some(p) = self.plugins.iter_mut().find(|p| p.id == id) {
            p.state = PluginState::Disabled;
        }
    }

    pub fn unload(&mut self, id: u64) {
        self.plugins.retain(|p| p.id != id);
    }

    pub fn get_hook_handlers(&self, hook: &str) -> Vec<&Plugin> {
        let mut handlers: Vec<&Plugin> = self
            .plugins
            .iter()
            .filter(|p| matches!(p.state, PluginState::Active) && p.hooks.iter().any(|h| h == hook))
            .collect();
        handlers.sort_by(|a, b| a.priority.cmp(&b.priority));
        handlers
    }

    pub fn active_count(&self) -> usize {
        self.plugins.iter().filter(|p| matches!(p.state, PluginState::Active)).count()
    }

    pub fn total(&self) -> usize {
        self.plugins.len()
    }
}
