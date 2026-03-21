extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct OrgChartChangeDetector {
    employees: Vec<String>,
    previous_state: Vec<String>,
}

impl OrgChartChangeDetector {
    pub fn new(employees: Vec<String>) -> Self {
        let previous_state = employees.clone();
        OrgChartChangeDetector {
            employees,
            previous_state,
        }
    }

    pub fn add_employee(&mut self, name: String) {
        if !self.employees.contains(&name) {
            self.employees.push(name);
        }
    }

    pub fn remove_employee(&mut self, name: &str) {
        if let Some(index) = self.employees.iter().position(|x| x == name) {
            self.employees.remove(index);
        }
    }

    pub fn has_changed(&self) -> bool {
        !self.employees.eq(&self.previous_state)
    }

    pub fn update_previous_state(&mut self) {
        self.previous_state = self.employees.clone();
    }

    pub fn list_employees(&self) -> Vec<String> {
        self.employees.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_org_chart_change_detector() {
        let mut detector = OrgChartChangeDetector::new(vec![
            String::from("Alice"),
            String::from("Bob"),
        ];

        assert_eq!(detector.list_employees(), vec!["Alice", "Bob"]);
        assert!(!detector.has_changed());

        detector.add_employee(String::from("Charlie"));
        assert_eq!(detector.list_employees(), vec!["Alice", "Bob", "Charlie"]);
        assert!(detector.has_changed());

        detector.update_previous_state();
        assert!(!detector.has_changed());

        detector.remove_employee("Bob");
        assert_eq!(detector.list_employees(), vec!["Alice", "Charlie"]);
        assert!(detector.has_changed());
    }
}
