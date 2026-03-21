extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_confirm_understand_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_confirm_understand_exit() {
    // Cleanup logic for the module
}

pub struct SpeechConfirmUnderstand {
    commands: Vec<String>,
    history: Vec<String>,
    max_history_size: usize,
}

impl SpeechConfirmUnderstand {
    pub fn new(max_history_size: usize) -> Self {
        SpeechConfirmUnderstand {
            commands: Vec::new(),
            history: Vec::new(),
            max_history_size,
        }
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn confirm_understanding(&self, input: &str) -> bool {
        for command in &self.commands {
            if input.contains(command) {
                return true;
            }
        }
        false
    }

    pub fn get_history(&self) -> Vec<String> {
        self.history.clone()
    }

    pub fn add_to_history(&mut self, entry: String) {
        if self.history.len() >= self.max_history_size {
            self.history.remove(0);
        }
        self.history.push(entry);
    }

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }
}

#[no_mangle]
pub extern "C" fn speech_confirm_understand_add_command(module: *mut SpeechConfirmUnderstand, command: *const u8, len: usize) -> i32 {
    if module.is_null() || command.is_null() {
        return -1;
    }

    let cmd_str = unsafe { core::slice::from_raw_parts(command, len) };
    let cmd_string = String::from_utf8_lossy(cmd_str).into_owned();

    unsafe { (*module).add_command(cmd_string); }
    0
}

#[no_mangle]
pub extern "C" fn speech_confirm_understand_confirm_understanding(module: *const SpeechConfirmUnderstand, input: *const u8, len: usize) -> i32 {
    if module.is_null() || input.is_null() {
        return -1;
    }

    let input_str = unsafe { core::slice::from_raw_parts(input, len) };
    let input_string = String::from_utf8_lossy(input_str).into_owned();

    unsafe { (*module).confirm_understanding(&input_string) as i32 }
}

#[no_mangle]
pub extern "C" fn speech_confirm_understand_get_history(module: *const SpeechConfirmUnderstand, history: *mut *const u8, sizes: *mut usize, count: usize) -> i32 {
    if module.is_null() || history.is_null() || sizes.is_null() {
        return -1;
    }

    let hist = unsafe { (*module).get_history() };
    let num_entries = core::cmp::min(hist.len(), count);

    for i in 0..num_entries {
        let entry = hist[i].as_bytes();
        unsafe {
            *history.add(i) = entry.as_ptr();
            *sizes.add(i) = entry.len();
        }
    }

    num_entries as i32
}

#[no_mangle]
pub extern "C" fn speech_confirm_understand_add_to_history(module: *mut SpeechConfirmUnderstand, entry: *const u8, len: usize) -> i32 {
    if module.is_null() || entry.is_null() {
        return -1;
    }

    let entry_str = unsafe { core::slice::from_raw_parts(entry, len) };
    let entry_string = String::from_utf8_lossy(entry_str).into_owned();

    unsafe { (*module).add_to_history(entry_string); }
    0
}

#[no_mangle]
pub extern "C" fn speech_confirm_understand_clear_commands(module: *mut SpeechConfirmUnderstand) -> i32 {
    if module.is_null() {
        return -1;
    }

    unsafe { (*module).clear_commands(); }
    0
}
