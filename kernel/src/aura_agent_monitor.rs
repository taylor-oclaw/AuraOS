extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Cleanup the module
}

struct AuraAgentMonitor {
    agents: Vec<String>,
    logs: Vec<String>,
}

impl AuraAgentMonitor {
    pub fn new() -> Self {
        AuraAgentMonitor {
            agents: Vec::new(),
            logs: Vec::new(),
        }
    }

    pub fn add_agent(&mut self, agent_name: &str) {
        self.agents.push(String::from(agent_name));
    }

    pub fn remove_agent(&mut self, agent_name: &str) -> bool {
        if let Some(index) = self.agents.iter().position(|x| x == agent_name) {
            self.agents.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_agents(&self) -> Vec<String> {
        self.agents.clone()
    }

    pub fn log_event(&mut self, event: &str) {
        self.logs.push(String::from(event));
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.logs.clone()
    }
}

#[no_mangle]
pub extern "C" fn aura_agent_monitor_new() -> *mut AuraAgentMonitor {
    Box::into_raw(Box::new(AuraAgentMonitor::new()))
}

#[no_mangle]
pub extern "C" fn aura_agent_monitor_add_agent(monitor: *mut AuraAgentMonitor, agent_name: *const u8, len: usize) {
    unsafe {
        let monitor = &mut *monitor;
        let agent_name_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(agent_name, len));
        monitor.add_agent(agent_name_str);
    }
}

#[no_mangle]
pub extern "C" fn aura_agent_monitor_remove_agent(monitor: *mut AuraAgentMonitor, agent_name: *const u8, len: usize) -> bool {
    unsafe {
        let monitor = &mut *monitor;
        let agent_name_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(agent_name, len));
        monitor.remove_agent(agent_name_str)
    }
}

#[no_mangle]
pub extern "C" fn aura_agent_monitor_list_agents(monitor: *const AuraAgentMonitor) -> *mut Vec<String> {
    unsafe {
        let monitor = &*monitor;
        Box::into_raw(Box::new(monitor.list_agents()))
    }
}

#[no_mangle]
pub extern "C" fn aura_agent_monitor_log_event(monitor: *mut AuraAgentMonitor, event: *const u8, len: usize) {
    unsafe {
        let monitor = &mut *monitor;
        let event_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(event, len));
        monitor.log_event(event_str);
    }
}

#[no_mangle]
pub extern "C" fn aura_agent_monitor_get_logs(monitor: *const AuraAgentMonitor) -> *mut Vec<String> {
    unsafe {
        let monitor = &*monitor;
        Box::into_raw(Box::new(monitor.get_logs()))
    }
}
