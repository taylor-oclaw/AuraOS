extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct Company {
    name: String,
    employees: Vec<String>,
    location: String,
    revenue: u64,
    is_publicly_traded: bool,
}

impl Company {
    pub fn new(name: &str, location: &str, revenue: u64) -> Self {
        Company {
            name: String::from(name),
            employees: Vec::new(),
            location: String::from(location),
            revenue,
            is_publicly_traded: false,
        }
    }

    pub fn add_employee(&mut self, employee_name: &str) {
        self.employees.push(String::from(employee_name));
    }

    pub fn remove_employee(&mut self, employee_name: &str) -> bool {
        if let Some(index) = self.employees.iter().position(|e| e == employee_name) {
            self.employees.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_employees_count(&self) -> usize {
        self.employees.len()
    }

    pub fn set_publicly_traded(&mut self, is_public: bool) {
        self.is_publicly_traded = is_public;
    }

    pub fn get_company_info(&self) -> String {
        let mut info = String::from("info");
        info.push_str(&String::from("info"));
        info.push_str(&String::from("info"));
        info.push_str(&String::from("info"));
        info.push_str("Employees:\n");
        for employee in &self.employees {
            info.push_str(&String::from("info"));
        }
        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_creation() {
        let company = Company::new("TechCorp", "Silicon Valley", 1000000);
        assert_eq!(company.name, "TechCorp");
        assert_eq!(company.location, "Silicon Valley");
        assert_eq!(company.revenue, 1_000_000);
        assert!(!company.is_publicly_traded);
    }

    #[test]
    fn test_add_remove_employee() {
        let mut company = Company::new("TechCorp", "Silicon Valley", 1000000);
        company.add_employee("Alice");
        company.add_employee("Bob");
        assert_eq!(company.get_employees_count(), 2);

        assert!(company.remove_employee("Alice"));
        assert!(!company.remove_employee("Charlie"));
        assert_eq!(company.get_employees_count(), 1);
    }

    #[test]
    fn test_set_publicly_traded() {
        let mut company = Company::new("TechCorp", "Silicon Valley", 1000000);
        company.set_publicly_traded(true);
        assert!(company.is_publicly_traded);

        company.set_publicly_traded(false);
        assert!(!company.is_publicly_traded);
    }

    #[test]
    fn test_get_company_info() {
        let mut company = Company::new("TechCorp", "Silicon Valley", 1000000);
        company.add_employee("Alice");
        company.set_publicly_traded(true);

        let info = company.get_company_info();
        assert!(info.contains("Company Name: TechCorp"));
        assert!(info.contains("Location: Silicon Valley"));
        assert!(info.contains("Revenue: $1000000"));
        assert!(info.contains("Publicly Traded: Yes"));
        assert!(info.contains("- Alice"));
    }
}
