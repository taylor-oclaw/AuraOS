extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct ResourceHandler {
    resources: Vec<String>,
}

impl ResourceHandler {
    pub fn new() -> Self {
        ResourceHandler {
            resources: Vec::new(),
        }
    }

    pub fn add_resource(&mut self, resource: String) {
        self.resources.push(resource);
    }

    pub fn remove_resource(&mut self, index: usize) -> Option<String> {
        if index < self.resources.len() {
            Some(self.resources.remove(index))
        } else {
            None
        }
    }

    pub fn get_resource(&self, index: usize) -> Option<&String> {
        self.resources.get(index)
    }

    pub fn list_resources(&self) -> &Vec<String> {
        &self.resources
    }

    pub fn count_resources(&self) -> usize {
        self.resources.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_handler() {
        let mut handler = ResourceHandler::new();
        assert_eq!(handler.count_resources(), 0);

        handler.add_resource(String::from("Resource1"));
        handler.add_resource(String::from("Resource2"));
        assert_eq!(handler.count_resources(), 2);

        assert_eq!(handler.get_resource(0), Some(&String::from("Resource1")));
        assert_eq!(handler.get_resource(1), Some(&String::from("Resource2")));

        let removed = handler.remove_resource(0);
        assert_eq!(removed, Some(String::from("Resource1")));
        assert_eq!(handler.count_resources(), 1);

        let resources = handler.list_resources();
        assert_eq!(resources.len(), 1);
        assert_eq!(&resources[0], "Resource2");
    }
}
