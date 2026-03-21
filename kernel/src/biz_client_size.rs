extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct BizClientSize {
    client_id: u32,
    size: usize,
    data: Vec<u8>,
    name: String,
    active: bool,
}

impl BizClientSize {
    pub fn new(client_id: u32, initial_size: usize, name: &str) -> Self {
        BizClientSize {
            client_id,
            size: initial_size,
            data: vec![0; initial_size],
            name: String::from(name),
            active: true,
        }
    }

    pub fn get_client_id(&self) -> u32 {
        self.client_id
    }

    pub fn set_size(&mut self, new_size: usize) {
        if new_size != self.size {
            self.data.resize(new_size, 0);
            self.size = new_size;
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn write_data(&mut self, offset: usize, data: &[u8]) -> Result<(), &'static str> {
        if offset + data.len() > self.size {
            Err("Data exceeds buffer size")
        } else {
            self.data[offset..offset + data.len()].copy_from_slice(data);
            Ok(())
        }
    }

    pub fn read_data(&self, offset: usize, length: usize) -> Result<Vec<u8>, &'static str> {
        if offset + length > self.size {
            Err("Data exceeds buffer size")
        } else {
            Ok(self.data[offset..offset + length].to_vec())
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}
