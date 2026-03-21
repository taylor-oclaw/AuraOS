extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut detector = RetirementDetector::new();
    detector.add_employee("Alice", 30);
    detector.add_employee("Bob", 45);
    detector.add_employee("Charlie", 60);

    println!("Retirement candidates: {:?}", detector.get_retirement_candidates(60));
}

pub struct RetirementDetector {
    employees: Vec<(String, u8)>,
}

impl RetirementDetector {
    pub fn new() -> Self {
        RetirementDetector {
            employees: Vec::new(),
        }
    }

    pub fn add_employee(&mut self, name: &str, age: u8) {
        self.employees.push((String::from(name), age));
    }

    pub fn remove_employee(&mut self, name: &str) {
        self.employees.retain(|(n, _)| n != name);
    }

    pub fn get_oldest_employee(&self) -> Option<&(String, u8)> {
        self.employees.iter().max_by_key(|&(_, age)| age)
    }

    pub fn get_youngest_employee(&self) -> Option<&(String, u8)> {
        self.employees.iter().min_by_key(|&(_, age)| age)
    }

    pub fn get_retirement_candidates(&self, retirement_age: u8) -> Vec<String> {
        self.employees
            .iter()
            .filter_map(|(name, &age)| if age >= retirement_age { Some(name.clone()) } else { None })
            .collect()
    }
}
