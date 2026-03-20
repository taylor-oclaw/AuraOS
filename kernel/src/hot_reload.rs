extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct WasmApp {
    pub id: u64,
    pub name: String,
    pub version: String,
    pub bytecode_hash: [u8; 16],
    pub loaded: bool,
    pub reload_count: u32
}

pub struct HotReloader {
    pub apps: Vec<WasmApp>,
    pub watch_paths: Vec<String>,
    pub auto_reload: bool,
    pub next_id: u64
}

impl HotReloader {
    pub fn new() -> Self {
        Self {
            apps: Vec::new(),
            watch_paths: Vec::new(),
            auto_reload: true,
            next_id: 1
        }
    }

    pub fn register_app(&mut self, name: &str, version: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.apps.push(WasmApp {
            id,
            name: String::from(name),
            version: String::from(version),
            bytecode_hash: [0; 16],
            loaded: true,
            reload_count: 0
        });
        id
    }

    pub fn reload(&mut self, id: u64, new_hash: [u8; 16]) -> bool {
        if let Some(app) = self.apps.iter_mut().find(|a| a.id == id) {
            if app.bytecode_hash != new_hash {
                app.bytecode_hash = new_hash;
                app.reload_count += 1;
                return true;
            }
        }
        false
    }

    pub fn watch(&mut self, path: &str) {
        self.watch_paths.push(String::from(path));
    }

    pub fn loaded_apps(&self) -> Vec<&WasmApp> {
        self.apps.iter().filter(|a| a.loaded).collect()
    }

    pub fn total_reloads(&self) -> u32 {
        self.apps.iter().map(|a| a.reload_count).sum()
    }
}
