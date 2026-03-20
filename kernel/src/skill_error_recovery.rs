extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

struct SkillErrorRecovery {
    errors: Vec<String>,
    recovery_steps: Vec<String>,
}

impl SkillErrorRecovery {
    pub fn new() -> Self {
        SkillErrorRecovery {
            errors: Vec::new(),
            recovery_steps: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn add_recovery_step(&mut self, step: String) {
        self.recovery_steps.push(step);
    }

    pub fn get_recovery_steps(&self) -> &Vec<String> {
        &self.recovery_steps
    }

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }
}
