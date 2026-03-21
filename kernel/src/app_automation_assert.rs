extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn app_automation_assert_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn app_automation_assert_exit() {
    // Cleanup logic for the module
}

pub struct AppAutomationAssert {
    assertions: Vec<String>,
}

impl AppAutomationAssert {
    pub fn new() -> Self {
        AppAutomationAssert {
            assertions: Vec::new(),
        }
    }

    pub fn add_assertion(&mut self, assertion: &str) {
        self.assertions.push(String::from(assertion));
    }

    pub fn get_all_assertions(&self) -> &[String] {
        &self.assertions
    }

    pub fn clear_assertions(&mut self) {
        self.assertions.clear();
    }

    pub fn has_failed_assertions(&self) -> bool {
        !self.assertions.is_empty()
    }

    pub fn count_assertions(&self) -> usize {
        self.assertions.len()
    }
}
