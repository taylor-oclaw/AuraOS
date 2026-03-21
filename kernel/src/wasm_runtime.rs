extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;

pub struct WasmModule {
    pub name: String,
    pub bytecode: Vec<u8>,
    pub memory_pages: u32,
    pub imports: Vec<WasmImport>,
    pub exports: Vec<WasmExport>,
    pub sandboxed: bool
}

pub struct WasmImport {
    pub module: String,
    pub name: String,
    pub kind: ImportKind
}

pub enum ImportKind {
    Function,
    Memory,
    Table,
    Global
}

pub struct WasmExport {
    pub name: String,
    pub kind: ExportKind
}

pub enum ExportKind {
    Function,
    Memory,
    Table,
    Global
}

pub struct WasmInstance {
    pub module_name: String,
    pub memory: Vec<u8>,
    pub stack: Vec<u64>,
    pub pc: usize,
    pub running: bool,
    pub exit_code: Option<i32>,
    pub capabilities: Vec<String>
}

pub struct WasmRuntime {
    pub modules: Vec<WasmModule>,
    pub instances: Vec<WasmInstance>,
    pub next_id: usize
}

impl WasmRuntime {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            instances: Vec::new(),
            next_id: 0
        }
    }

    pub fn load_module(&mut self, name: &str, bytecode: Vec<u8>) -> usize {
        let id = self.modules.len();
        self.modules.push(WasmModule {
            name: String::from(name),
            bytecode,
            memory_pages: 16,
            imports: Vec::new(),
            exports: Vec::new(),
            sandboxed: true
        };
        id
    }

    pub fn instantiate(&mut self, module_idx: usize) -> Option<usize> {
        if module_idx >= self.modules.len() {
            return None;
        }
        let module = &self.modules[module_idx];
        let mem_size = module.memory_pages as usize * 65536;
        let id = self.instances.len();
        self.instances.push(WasmInstance {
            module_name: module.name.clone(),
            memory: vec![0u8; mem_size],
            stack: Vec::new(),
            pc: 0,
            running: false,
            exit_code: None,
            capabilities: Vec::new()
        };
        Some(id)
    }

    pub fn grant_capability(&mut self, instance_idx: usize, cap: &str) {
        if let Some(inst) = self.instances.get_mut(instance_idx) {
            inst.capabilities.push(String::from(cap));
        }
    }

    pub fn has_capability(&self, instance_idx: usize, cap: &str) -> bool {
        self.instances.get(instance_idx).map(|i| i.capabilities.iter().any(|c| c == cap)).unwrap_or(false)
    }

    pub fn start(&mut self, instance_idx: usize) -> bool {
        if let Some(inst) = self.instances.get_mut(instance_idx) {
            inst.running = true;
            true
        } else {
            false
        }
    }

    pub fn stop(&mut self, instance_idx: usize, exit_code: i32) {
        if let Some(inst) = self.instances.get_mut(instance_idx) {
            inst.running = false;
            inst.exit_code = Some(exit_code);
        }
    }
))}
