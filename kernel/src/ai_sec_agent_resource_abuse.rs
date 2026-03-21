extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AiSecAgentResourceAbuse {
    resources: Vec<String>,
    max_resources: usize,
}

impl AiSecAgentResourceAbuse {
    pub fn new(max_resources: usize) -> Self {
        AiSecAgentResourceAbuse {
            resources: Vec::new(),
            max_resources,
        }
    }

    pub fn add_resource(&mut self, resource: String) -> bool {
        if self.resources.len() < self.max_resources {
            self.resources.push(resource);
            true
        } else {
            false
        }
    }

    pub fn remove_resource(&mut self, index: usize) -> Option<String> {
        if index < self.resources.len() {
            Some(self.resources.remove(index))
        } else {
            None
        }
    }

    pub fn get_resources(&self) -> &[String] {
        &self.resources
    }

    pub fn is_resource_limit_reached(&self) -> bool {
        self.resources.len() >= self.max_resources
    }

    pub fn clear_resources(&mut self) {
        self.resources.clear();
    }
}
