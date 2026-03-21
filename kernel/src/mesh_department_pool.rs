extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshDepartmentPool {
    departments: Vec<String>,
}

impl MeshDepartmentPool {
    pub fn new() -> Self {
        MeshDepartmentPool {
            departments: Vec::new(),
        }
    }

    pub fn add_department(&mut self, name: &str) {
        if !self.departments.contains(&String::from(name)) {
            self.departments.push(String::from(name));
        }
    }

    pub fn remove_department(&mut self, name: &str) {
        self.departments.retain(|d| d != name);
    }

    pub fn list_departments(&self) -> Vec<String> {
        self.departments.clone()
    }

    pub fn department_exists(&self, name: &str) -> bool {
        self.departments.contains(&String::from(name))
    }

    pub fn count_departments(&self) -> usize {
        self.departments.len()
    }
}
