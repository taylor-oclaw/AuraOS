extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_personal_mesh_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_personal_mesh_exit() {
    // Cleanup logic for the module
}

pub struct MeshNode {
    id: u32,
    name: String,
    neighbors: Vec<u32>,
    data: Vec<u8>,
    status: NodeStatus,
}

#[derive(Debug, PartialEq)]
enum NodeStatus {
    Active,
    Inactive,
    Error,
}

impl MeshNode {
    pub fn new(id: u32, name: &str) -> Self {
        MeshNode {
            id,
            name: String::from(name),
            neighbors: Vec::new(),
            data: Vec::new(),
            status: NodeStatus::Active,
        }
    }

    pub fn add_neighbor(&mut self, neighbor_id: u32) {
        if !self.neighbors.contains(&neighbor_id) {
            self.neighbors.push(neighbor_id);
        }
    }

    pub fn remove_neighbor(&mut self, neighbor_id: u32) {
        self.neighbors.retain(|&id| id != neighbor_id);
    }

    pub fn set_data(&mut self, data: &[u8]) {
        self.data.clear();
        self.data.extend_from_slice(data);
    }

    pub fn get_status(&self) -> NodeStatus {
        self.status
    }

    pub fn update_status(&mut self, new_status: NodeStatus) {
        self.status = new_status;
    }
}

#[no_mangle]
pub extern "C" fn mesh_personal_mesh_create_node(id: u32, name: *const u8, name_len: usize) -> *mut MeshNode {
    let name_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(name, name_len)) };
    Box::into_raw(Box::new(MeshNode::new(id, name_str)))
}

#[no_mangle]
pub extern "C" fn mesh_personal_mesh_destroy_node(node: *mut MeshNode) {
    unsafe { drop(Box::from_raw(node)); }
}
