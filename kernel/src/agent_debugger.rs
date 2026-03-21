extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn agent_debugger_init() {
    // Initialization logic for the agent debugger module
}

pub extern "C" fn agent_debugger_exit() {
    // Cleanup logic for the agent debugger module
}

pub struct AgentDebugger {
    logs: Vec<String>,
    enabled: bool,
}

impl AgentDebugger {
    pub fn new() -> Self {
        AgentDebugger {
            logs: Vec::new(),
            enabled: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn log_message(&mut self, message: &str) {
        if self.enabled {
            self.logs.push(String::from(message));
        }
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.logs.clone()
    }
}

pub extern "C" fn agent_debugger_log_message(message: *const u8, length: usize) {
    unsafe {
        let slice = core::slice::from_raw_parts(message, length);
        if let Ok(str_slice) = core::str::from_utf8(slice) {
            let mut debugger = AgentDebugger::new();
            debugger.log_message(str_slice);
            // Here you would typically store or process the logs as needed
        }
    }
}
