extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_right_to_export_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_right_to_export_exit() {
    // Cleanup logic for the module
}

pub struct MeshRightToExport {
    data: Vec<u8>,
    name: String,
    version: u32,
    is_active: bool,
    error_code: i32,
}

impl MeshRightToExport {
    pub fn new(name: &str, version: u32) -> Self {
        MeshRightToExport {
            data: Vec::new(),
            name: String::from(name),
            version,
            is_active: false,
            error_code: 0,
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn set_error_code(&mut self, code: i32) {
        self.error_code = code;
    }

    pub fn get_error_code(&self) -> i32 {
        self.error_code
    }
}
