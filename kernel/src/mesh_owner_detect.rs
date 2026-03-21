extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_owner_detect_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_owner_detect_exit() {
    // Cleanup logic for the module
}

pub struct MeshOwnerDetect {
    owners: Vec<String>,
    devices: Vec<String>,
}

impl MeshOwnerDetect {
    pub fn new() -> Self {
        MeshOwnerDetect {
            owners: Vec::new(),
            devices: Vec::new(),
        }
    }

    pub fn add_owner(&mut self, owner: String) {
        if !self.owners.contains(&owner) {
            self.owners.push(owner);
        }
    }

    pub fn remove_owner(&mut self, owner: &str) {
        self.owners.retain(|o| o != owner);
    }

    pub fn add_device(&mut self, device: String) {
        if !self.devices.contains(&device) {
            self.devices.push(device);
        }
    }

    pub fn remove_device(&mut self, device: &str) {
        self.devices.retain(|d| d != device);
    }

    pub fn list_owners(&self) -> Vec<String> {
        self.owners.clone()
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_owner_detect() {
        let mut mod_instance = MeshOwnerDetect::new();
        mod_instance.add_owner(String::from("Alice"));
        mod_instance.add_device(String::from("Device1"));

        assert_eq!(mod_instance.list_owners(), vec![String::from("Alice")]);
        assert_eq!(mod_instance.list_devices(), vec![String::from("Device1")]);

        mod_instance.remove_owner("Alice");
        mod_instance.remove_device("Device1");

        assert_eq!(mod_instance.list_owners().len(), 0);
        assert_eq!(mod_instance.list_devices().len(), 0);
    }
}
