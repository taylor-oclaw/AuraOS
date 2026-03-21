extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_mesh_pci_boundary() {
    // Initialization logic for the mesh_pci_boundary module
}

#[no_mangle]
pub extern "C" fn exit_mesh_pci_boundary() {
    // Cleanup logic for the mesh_pci_boundary module
}

pub struct MeshPCIBoundary {
    devices: Vec<String>,
    active_device_index: usize,
}

impl MeshPCIBoundary {
    pub fn new() -> Self {
        MeshPCIBoundary {
            devices: Vec::new(),
            active_device_index: 0,
        }
    }

    pub fn add_device(&mut self, device_name: String) {
        self.devices.push(device_name);
    }

    pub fn remove_device(&mut self, device_name: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_active_device(&self) -> Option<&String> {
        self.devices.get(self.active_device_index)
    }

    pub fn set_active_device(&mut self, device_name: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.active_device_index = index;
            true
        } else {
            false
        }
    }

    pub fn list_devices(&self) -> Vec<&String> {
        self.devices.iter().collect()
    }
}
