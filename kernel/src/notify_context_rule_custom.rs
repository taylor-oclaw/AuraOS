extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn notify_context_rule_custom_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn notify_context_rule_custom_exit() {
    // Cleanup logic for the module
}

pub struct NotifyContextRuleCustom {
    rules: Vec<String>,
    active: bool,
}

impl NotifyContextRuleCustom {
    pub fn new() -> Self {
        NotifyContextRuleCustom {
            rules: Vec::new(),
            active: false,
        }
    }

    pub fn add_rule(&mut self, rule: String) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, index: usize) -> Option<String> {
        if index < self.rules.len() {
            Some(self.rules.remove(index))
        } else {
            None
        }
    }

    pub fn list_rules(&self) -> Vec<&str> {
        self.rules.iter().map(|rule| rule.as_str()).collect()
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

#[no_mangle]
pub extern "C" fn notify_context_rule_custom_add_rule(rule: *const u8, len: usize) -> i32 {
    let rule_str = unsafe { core::slice::from_raw_parts(rule, len) };
    if let Ok(rule_string) = String::from_utf8(rule_str.to_vec()) {
        let module = NotifyContextRuleCustom::new();
        module.add_rule(rule_string);
        0
    } else {
        -1
    }
}

#[no_mangle]
pub extern "C" fn notify_context_rule_custom_remove_rule(index: usize) -> i32 {
    let mut module = NotifyContextRuleCustom::new();
    if module.remove_rule(index).is_some() {
        0
    } else {
        -1
    }
}

#[no_mangle]
pub extern "C" fn notify_context_rule_custom_list_rules(buffer: *mut u8, buffer_len: usize) -> i32 {
    let mut module = NotifyContextRuleCustom::new();
    let rules = module.list_rules();
    let total_len: usize = rules.iter().map(|rule| rule.len() + 1).sum(); // +1 for null terminator

    if buffer_len < total_len {
        return -1;
    }

    let mut offset = 0;
    for rule in rules {
        unsafe {
            core::ptr::copy_nonoverlapping(rule.as_ptr(), buffer.offset(offset as isize), rule.len());
            *buffer.offset((offset + rule.len()) as isize) = 0; // null terminator
        }
        offset += rule.len() + 1;
    }

    total_len as i32
}

#[no_mangle]
pub extern "C" fn notify_context_rule_custom_activate() {
    let mut module = NotifyContextRuleCustom::new();
    module.activate();
}

#[no_mangle]
pub extern "C" fn notify_context_rule_custom_deactivate() {
    let mut module = NotifyContextRuleCustom::new();
    module.deactivate();
}

#[no_mangle]
pub extern "C" fn notify_context_rule_custom_is_active() -> i32 {
    let module = NotifyContextRuleCustom::new();
    if module.is_active() {
        1
    } else {
        0
    }
}
