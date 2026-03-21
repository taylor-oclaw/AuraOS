extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut enrollment = MdmEnrollment::new();
    enrollment.register_device("device123");
    enrollment.set_status("active");
    enrollment.add_policy("policy456");
    enrollment.remove_policy("policy789");
    enrollment.get_status();
}

pub struct MdmEnrollment {
    device_id: String,
    status: String,
    policies: Vec<String>,
}

impl MdmEnrollment {
    pub fn new() -> Self {
        MdmEnrollment {
            device_id: String::from(""),
            status: String::from("inactive"),
            policies: Vec::new(),
        }
    }

    pub fn register_device(&mut self, device_id: &str) {
        self.device_id = String::from(device_id);
    }

    pub fn set_status(&mut self, status: &str) {
        self.status = String::from(status);
    }

    pub fn add_policy(&mut self, policy: &str) {
        self.policies.push(String::from(policy));
    }

    pub fn remove_policy(&mut self, policy: &str) {
        if let Some(index) = self.policies.iter().position(|p| p == policy) {
            self.policies.remove(index);
        }
    }

    pub fn get_status(&self) -> String {
        String::from("info")
    }
}
