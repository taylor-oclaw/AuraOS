extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct BackupManifoldSelective {
    manifold: Vec<u8>,
}

impl BackupManifoldSelective {
    pub fn new(manifold: Vec<u8>) -> Self {
        BackupManifoldSelective { manifold }
    }

    pub fn get_manifold(&self) -> &Vec<u8> {
        &self.manifold
    }

    pub fn set_manifold(&mut self, manifold: Vec<u8>) {
        self.manifold = manifold;
    }

    pub fn add_to_manifold(&mut self, data: Vec<u8>) {
        let mut new_manifold = self.manifold.clone();
        new_manifold.extend(data);
        self.manifold = new_manifold;
    }

    pub fn remove_from_manifold(&mut self, data: Vec<u8>) -> bool {
        if self.manifold.contains(&data[0]) {
            let mut new_manifold = self.manifold.clone();
            new_manifold.retain(|x| x != &data[0]);
            self.manifold = new_manifold;
            true
        } else {
            false
        }
    }

    pub fn get_size_of_manifold(&self) -> usize {
        self.manifold.len()
    }
}