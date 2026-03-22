extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct BackupRestoreVoid {
    data: Vec<u8>,
}

impl BackupRestoreVoid {
    pub fn new() -> Self {
        BackupRestoreVoid { data: Vec::new() }
    }

    pub fn backup(&mut self, source: &[u8]) {
        self.data.clear();
        self.data.extend_from_slice(source);
    }

    pub fn restore(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_backup(&mut self) {
        self.data.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}