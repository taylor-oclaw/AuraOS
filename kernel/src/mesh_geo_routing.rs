extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() {}

struct MeshGeoRouting {
    nodes: Vec<String>,
    routes: Vec<(String, String)>,
}

impl MeshGeoRouting {
    pub fn new() -> Self {
        MeshGeoRouting {
            nodes: Vec::new(),
            routes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_id: &str) {
        if !self.nodes.contains(&node_id.to_string()) {
            self.nodes.push(node_id.to_string());
        }
    }

    pub fn remove_node(&mut self, node_id: &str) {
        self.nodes.retain(|n| n != node_id);
        self.routes.retain(|&(ref from, ref to)| from != node_id && to != node_id);
    }

    pub fn add_route(&mut self, from: &str, to: &str) {
        if self.nodes.contains(&from.to_string()) && self.nodes.contains(&to.to_string()) {
            self.routes.push((from.to_string(), to.to_string()));
        }
    }

    pub fn remove_route(&mut self, from: &str, to: &str) {
        self.routes.retain(|&(ref f, ref t)| f != from || t != to);
    }

    pub fn get_routes_from(&self, node_id: &str) -> Vec<String> {
        self.routes
            .iter()
            .filter_map(|&(ref from, ref to)| if from == node_id { Some(to.clone()) } else { None })
            .collect()
    }
}
