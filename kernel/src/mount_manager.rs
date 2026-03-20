extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MountManager {
    mounts: Vec<String>,
}

impl MountManager {
    pub fn new() -> Self {
        MountManager {
            mounts: Vec::new(),
        }
    }

    pub fn mount(&mut self, path: &str) -> Result<(), &'static str> {
        if self.mounts.contains(&String::from(path)) {
            Err("Mount point already exists")
        } else {
            self.mounts.push(String::from(path));
            Ok(())
        }
    }

    pub fn unmount(&mut self, path: &str) -> Result<(), &'static str> {
        match self.mounts.iter().position(|p| p == path) {
            Some(index) => {
                self.mounts.remove(index);
                Ok(())
            }
            None => Err("Mount point not found"),
        }
    }

    pub fn list_mounts(&self) -> Vec<String> {
        self.mounts.clone()
    }

    pub fn is_mounted(&self, path: &str) -> bool {
        self.mounts.contains(&String::from(path))
    }

    pub fn count_mounts(&self) -> usize {
        self.mounts.len()
    }
}
