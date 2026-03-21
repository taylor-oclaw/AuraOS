extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let policy = CorporatePolicy::new();
    policy.apply_policy();
    loop {}
}

pub struct CorporatePolicy {
    rules: Vec<String>,
    employees: Vec<String>,
}

impl CorporatePolicy {
    pub fn new() -> Self {
        CorporatePolicy {
            rules: Vec::new(),
            employees: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: &str) {
        self.rules.push(String::from(rule));
    }

    pub fn remove_rule(&mut self, rule_index: usize) -> Option<String> {
        if rule_index < self.rules.len() {
            Some(self.rules.remove(rule_index))
        } else {
            None
        }
    }

    pub fn add_employee(&mut self, employee_name: &str) {
        self.employees.push(String::from(employee_name));
    }

    pub fn remove_employee(&mut self, employee_index: usize) -> Option<String> {
        if employee_index < self.employees.len() {
            Some(self.employees.remove(employee_index))
        } else {
            None
        }
    }

    pub fn apply_policy(&self) {
        // Simulate applying the policy to employees
        for rule in &self.rules {
            println!("Applying rule: {}", rule);
        }
        for employee in &self.employees {
            println!("Employee affected by policy: {}", employee);
        }
    }
}
