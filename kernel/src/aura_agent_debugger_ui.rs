extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

pub struct AuraAgentDebuggerUI {
    commands: Vec<String>,
    history: Vec<String>,
    current_command: String,
}

impl AuraAgentDebuggerUI {
    pub fn new() -> Self {
        AuraAgentDebuggerUI {
            commands: Vec::new(),
            history: Vec::new(),
            current_command: String::new(),
        }
    }

    pub fn add_command(&mut self, command: &str) {
        let cmd = String::from(command);
        self.commands.push(cmd.clone());
        self.history.push(cmd);
    }

    pub fn get_commands(&self) -> &[String] {
        &self.commands
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    pub fn set_current_command(&mut self, command: &str) {
        self.current_command = String::from(command);
    }

    pub fn get_current_command(&self) -> &str {
        &self.current_command
    }
}

pub extern "C" fn aura_agent_debugger_ui_new() -> *mut AuraAgentDebuggerUI {
    Box::into_raw(Box::new(AuraAgentDebuggerUI::new()))
}

pub extern "C" fn aura_agent_debugger_ui_add_command(ui: *mut AuraAgentDebuggerUI, command: *const u8) {
    unsafe {
        let ui = &mut *ui;
        let c_str = core::ffi::CStr::from_ptr(command as *const _);
        if let Ok(cmd_str) = c_str.to_str() {
            ui.add_command(cmd_str);
        }
    }
}

pub extern "C" fn aura_agent_debugger_ui_get_commands(ui: *mut AuraAgentDebuggerUI, commands: *mut *const u8, count: *mut usize) {
    unsafe {
        let ui = &*ui;
        let cmd_count = ui.get_commands().len();
        *count = cmd_count;
        if cmd_count > 0 {
            let ptrs: Vec<*const u8> = ui.get_commands()
                .iter()
                .map(|cmd| cmd.as_ptr())
                .collect();
            core::ptr::copy_nonoverlapping(ptrs.as_ptr(), commands, cmd_count);
        }
    }
}

pub extern "C" fn aura_agent_debugger_ui_get_history(ui: *mut AuraAgentDebuggerUI, history: *mut *const u8, count: *mut usize) {
    unsafe {
        let ui = &*ui;
        let hist_count = ui.get_history().len();
        *count = hist_count;
        if hist_count > 0 {
            let ptrs: Vec<*const u8> = ui.get_history()
                .iter()
                .map(|cmd| cmd.as_ptr())
                .collect();
            core::ptr::copy_nonoverlapping(ptrs.as_ptr(), history, hist_count);
        }
    }
}

pub extern "C" fn aura_agent_debugger_ui_set_current_command(ui: *mut AuraAgentDebuggerUI, command: *const u8) {
    unsafe {
        let ui = &mut *ui;
        let c_str = core::ffi::CStr::from_ptr(command as *const _);
        if let Ok(cmd_str) = c_str.to_str() {
            ui.set_current_command(cmd_str);
        }
    }
}

pub extern "C" fn aura_agent_debugger_ui_get_current_command(ui: *mut AuraAgentDebuggerUI, command: *mut u8, size: usize) -> usize {
    unsafe {
        let ui = &*ui;
        let cmd_str = ui.get_current_command();
        let len = cmd_str.len().min(size - 1);
        core::ptr::copy_nonoverlapping(cmd_str.as_ptr(), command, len);
        *(command.offset(len as isize)) = 0; // Null-terminate the string
        len
    }
}

pub extern "C" fn aura_agent_debugger_ui_free(ui: *mut AuraAgentDebuggerUI) {
    unsafe {
        drop(Box::from_raw(ui));
    }
}
