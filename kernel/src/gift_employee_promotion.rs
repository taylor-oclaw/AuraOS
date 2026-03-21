extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct EmployeePromotion {
    employees: Vec<String>,
    promotions: Vec<String>,
}

impl EmployeePromotion {
    pub fn new() -> Self {
        EmployeePromotion {
            employees: Vec::new(),
            promotions: Vec::new(),
        }
    }

    pub fn add_employee(&mut self, name: &str) {
        self.employees.push(String::from(name));
    }

    pub fn remove_employee(&mut self, name: &str) -> bool {
        if let Some(index) = self.employees.iter().position(|e| e == name) {
            self.employees.remove(index);
            true
        } else {
            false
        }
    }

    pub fn promote_employee(&mut self, employee_name: &str, promotion_title: &str) -> bool {
        if let Some(index) = self.employees.iter().position(|e| e == employee_name) {
            self.promotions.push(format!("{} - {}", employee_name, promotion_title));
            true
        } else {
            false
        }
    }

    pub fn list_employees(&self) -> Vec<String> {
        self.employees.clone()
    }

    pub fn list_promotions(&self) -> Vec<String> {
        self.promotions.clone()
    }
}
