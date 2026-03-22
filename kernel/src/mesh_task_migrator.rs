extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshTaskMigrator {
    task_id: u32,
    mesh_id: u32,
    current_node: u32,
}

impl MeshTaskMigrator {
    pub fn new(task_id: u32, mesh_id: u32) -> Self {
        MeshTaskMigrator {
            task_id,
            mesh_id,
            current_node: 0,
        }
    }

    pub fn get_task_id(&self) -> u32 {
        self.task_id
    }

    pub fn get_mesh_id(&self) -> u32 {
        self.mesh_id
    }

    pub fn migrate_to_node(&mut self, node_id: u32) {
        if node_id != self.current_node {
            self.current_node = node_id;
            // Simulate migration by printing a message (no println! in kernel)
            print!("Task {} migrated to node {}\n", self.task_id, node_id);
        }
    }

    pub fn get_current_node(&self) -> u32 {
        self.current_node
    }

    pub fn update_mesh_id(&mut self, new_mesh_id: u32) {
        self.mesh_id = new_mesh_id;
    }
}