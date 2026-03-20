extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentMetaLearning {
    name: String,
    version: u32,
    capabilities: Vec<String>,
    learning_rate: f32,
    is_active: bool,
}

impl AgentMetaLearning {
    pub fn new(name: &str, version: u32) -> Self {
        AgentMetaLearning {
            name: String::from(name),
            version,
            capabilities: Vec::new(),
            learning_rate: 0.1,
            is_active: true,
        }
    }

    pub fn add_capability(&mut self, capability: &str) {
        self.capabilities.push(String::from(capability));
    }

    pub fn remove_capability(&mut self, capability: &str) {
        self.capabilities.retain(|c| c != capability);
    }

    pub fn get_capabilities(&self) -> Vec<String> {
        self.capabilities.clone()
    }

    pub fn update_learning_rate(&mut self, new_rate: f32) {
        if new_rate > 0.0 && new_rate < 1.0 {
            self.learning_rate = new_rate;
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

pub extern "C" fn create_agent(name: *const u8, version: u32) -> *mut AgentMetaLearning {
    let name_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(name, 0)) };
    Box::into_raw(Box::new(AgentMetaLearning::new(name_str, version)))
}

pub extern "C" fn add_capability(agent: *mut AgentMetaLearning, capability: *const u8) {
    let capability_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(capability, 0)) };
    unsafe { (*agent).add_capability(capability_str); }
}

pub extern "C" fn remove_capability(agent: *mut AgentMetaLearning, capability: *const u8) {
    let capability_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(capability, 0)) };
    unsafe { (*agent).remove_capability(capability_str); }
}

pub extern "C" fn get_capabilities(agent: *const AgentMetaLearning) -> Vec<String> {
    unsafe { (*agent).get_capabilities() }
}

pub extern "C" fn update_learning_rate(agent: *mut AgentMetaLearning, new_rate: f32) {
    unsafe { (*agent).update_learning_rate(new_rate); }
}

pub extern "C" fn deactivate_agent(agent: *mut AgentMetaLearning) {
    unsafe { (*agent).deactivate(); }
}
