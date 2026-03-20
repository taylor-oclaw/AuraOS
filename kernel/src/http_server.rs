extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct HttpServer {
    port: u16,
    routes: Vec<(String, String)>,
}

impl HttpServer {
    pub fn new(port: u16) -> Self {
        HttpServer {
            port,
            routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, path: &str, response: &str) {
        self.routes.push((String::from(path), String::from(response)));
    }

    pub fn remove_route(&mut self, path: &str) -> bool {
        let index = self.routes.iter().position(|&(ref p, _)| p == path);
        if let Some(i) = index {
            self.routes.remove(i);
            true
        } else {
            false
        }
    }

    pub fn get_response(&self, path: &str) -> Option<&String> {
        for (p, response) in &self.routes {
            if p == path {
                return Some(response);
            }
        }
        None
    }

    pub fn list_routes(&self) -> Vec<&String> {
        self.routes.iter().map(|(path, _)| path).collect()
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}
