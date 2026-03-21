extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mdm_data_boundary_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mdm_data_boundary_exit() {
    // Cleanup logic for the module
}

pub struct MdmDataBoundary {
    data: Vec<u8>,
    capacity: usize,
}

impl MdmDataBoundary {
    pub fn new(capacity: usize) -> Self {
        MdmDataBoundary {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn add_data(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        if self.data.len() + bytes.len() > self.capacity {
            Err("Capacity exceeded")
        } else {
            self.data.extend_from_slice(bytes);
            Ok(())
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn data_size(&self) -> usize {
        self.data.len()
    }

    pub fn remaining_capacity(&self) -> usize {
        self.capacity - self.data.len()
    }
}

#[no_mangle]
pub extern "C" fn mdm_data_boundary_add_data(mdb: *mut MdmDataBoundary, bytes: *const u8, len: usize) -> i32 {
    unsafe {
        let mdb = &mut *mdb;
        let data_slice = core::slice::from_raw_parts(bytes, len);
        match mdb.add_data(data_slice) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }
}

#[no_mangle]
pub extern "C" fn mdm_data_boundary_get_data_size(mdb: *const MdmDataBoundary) -> usize {
    unsafe { (*mdb).data_size() }
}

#[no_mangle]
pub extern "C" fn mdm_data_boundary_remaining_capacity(mdb: *const MdmDataBoundary) -> usize {
    unsafe { (*mdb).remaining_capacity() }
}
