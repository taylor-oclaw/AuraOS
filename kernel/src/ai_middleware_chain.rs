extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AIMiddlewareChain {
    middlewares: Vec<String>,
}

impl AIMiddlewareChain {
    pub fn new() -> Self {
        AIMiddlewareChain {
            middlewares: Vec::new(),
        }
    }

    pub fn add_middleware(&mut self, middleware_name: &str) {
        self.middlewares.push(String::from(middleware_name));
    }

    pub fn remove_middleware(&mut self, middleware_name: &str) -> bool {
        if let Some(index) = self.middlewares.iter().position(|m| m == middleware_name) {
            self.middlewares.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_middlewares(&self) -> Vec<String> {
        self.middlewares.clone()
    }

    pub fn has_middleware(&self, middleware_name: &str) -> bool {
        self.middlewares.contains(&String::from(middleware_name))
    }

    pub fn clear_middlewares(&mut self) {
        self.middlewares.clear();
    }
}
