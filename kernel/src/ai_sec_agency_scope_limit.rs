extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut agency = AISecAgencyScopeLimit::new();
    agency.add_scope("network".to_string());
    agency.add_scope("storage".to_string());
    agency.remove_scope("storage".to_string());
    agency.list_scopes();
    if agency.is_within_scope("network") {
        // Perform network-related operations
    }
    loop {}
}

pub struct AISecAgencyScopeLimit {
    scopes: Vec<String>,
}

impl AISecAgencyScopeLimit {
    pub fn new() -> Self {
        AISecAgencyScopeLimit { scopes: Vec::new() }
    }

    pub fn add_scope(&mut self, scope: String) {
        if !self.scopes.contains(&scope) {
            self.scopes.push(scope);
        }
    }

    pub fn remove_scope(&mut self, scope: String) {
        self.scopes.retain(|s| s != &scope);
    }

    pub fn list_scopes(&self) -> Vec<String> {
        self.scopes.clone()
    }

    pub fn is_within_scope(&self, scope: &str) -> bool {
        self.scopes.contains(&String::from(scope))
    }

    pub fn clear_scopes(&mut self) {
        self.scopes.clear();
    }
}
