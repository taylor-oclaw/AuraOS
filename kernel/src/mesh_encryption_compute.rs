extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_encryption_compute_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_encryption_compute_exit() {
    // Cleanup logic for the module
}

pub struct MeshEncryptionCompute {
    key: Vec<u8>,
    data: Vec<u8>,
}

impl MeshEncryptionCompute {
    pub fn new(key: Vec<u8>, data: Vec<u8>) -> Self {
        MeshEncryptionCompute { key, data }
    }

    pub fn encrypt(&mut self) {
        // Simple XOR encryption for demonstration purposes
        for i in 0..self.data.len() {
            self.data[i] ^= self.key[i % self.key.len()];
        }
    }

    pub fn decrypt(&mut self) {
        // Decrypting using the same XOR logic (since XOR is its own inverse)
        for i in 0..self.data.len() {
            self.data[i] ^= self.key[i % self.key.len()];
        }
    }

    pub fn set_key(&mut self, key: Vec<u8>) {
        self.key = key;
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = 0;
        }
    }
}

#[no_mangle]
pub extern "C" fn mesh_encryption_compute_encrypt(ptr: *mut MeshEncryptionCompute) {
    unsafe {
        if let Some(compute) = ptr.as_mut() {
            compute.encrypt();
        }
    }
}

#[no_mangle]
pub extern "C" fn mesh_encryption_compute_decrypt(ptr: *mut MeshEncryptionCompute) {
    unsafe {
        if let Some(compute) = ptr.as_mut() {
            compute.decrypt();
        }
    }
}

#[no_mangle]
pub extern "C" fn mesh_encryption_compute_set_key(ptr: *mut MeshEncryptionCompute, key_ptr: *const u8, key_len: usize) {
    unsafe {
        if let Some(compute) = ptr.as_mut() {
            let key_slice = core::slice::from_raw_parts(key_ptr, key_len);
            compute.set_key(key_slice.to_vec());
        }
    }
}

#[no_mangle]
pub extern "C" fn mesh_encryption_compute_get_data(ptr: *const MeshEncryptionCompute, data_out: *mut u8, max_len: usize) -> usize {
    unsafe {
        if let Some(compute) = ptr.as_ref() {
            let data_slice = compute.get_data();
            let len_to_copy = core::cmp::min(data_slice.len(), max_len);
            core::ptr::copy_nonoverlapping(data_slice.as_ptr(), data_out, len_to_copy);
            len_to_copy
        } else {
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn mesh_encryption_compute_clear_data(ptr: *mut MeshEncryptionCompute) {
    unsafe {
        if let Some(compute) = ptr.as_mut() {
            compute.clear_data();
        }
    }
}
