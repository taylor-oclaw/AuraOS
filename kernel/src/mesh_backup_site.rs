extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct MeshBackupSite {
    site_name: String,
    backup_nodes: Vec<String>,
    data_size: usize,
    last_backup_time: u64,
    is_active: bool,
}

impl MeshBackupSite {
    pub fn new(site_name: &str) -> Self {
        MeshBackupSite {
            site_name: String::from(site_name),
            backup_nodes: Vec::new(),
            data_size: 0,
            last_backup_time: 0,
            is_active: true,
        }
    }

    pub fn add_node(&mut self, node_name: &str) {
        self.backup_nodes.push(String::from(node_name));
    }

    pub fn remove_node(&mut self, node_name: &str) -> bool {
        if let Some(index) = self.backup_nodes.iter().position(|n| n == node_name) {
            self.backup_nodes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn update_data_size(&mut self, new_size: usize) {
        self.data_size = new_size;
    }

    pub fn perform_backup(&mut self, current_time: u64) -> bool {
        if self.is_active && self.backup_nodes.len() > 0 {
            self.last_backup_time = current_time;
            true
        } else {
            false
        }
    }

    pub fn get_status(&self) -> String {
        format!(
            "Site Name: {}, Nodes: {:?}, Data Size: {}, Last Backup Time: {}, Active: {}",
            self.site_name,
            self.backup_nodes,
            self.data_size,
            self.last_backup_time,
            self.is_active
        )
    }
}
