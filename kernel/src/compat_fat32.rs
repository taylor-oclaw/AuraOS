extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CompatFat32 {
    // Example fields for a FAT32 filesystem
    pub volume_label: String,
    pub total_sectors: u32,
    pub free_sectors: u32,
    pub sectors_per_cluster: u8,
}

impl CompatFat32 {
    pub fn new(volume_label: &str, total_sectors: u32, free_sectors: u32, sectors_per_cluster: u8) -> Self {
        CompatFat32 {
            volume_label: String::from(volume_label),
            total_sectors,
            free_sectors,
            sectors_per_cluster,
        }
    }

    pub fn get_volume_label(&self) -> &str {
        &self.volume_label
    }

    pub fn set_volume_label(&mut self, new_label: &str) {
        self.volume_label = String::from(new_label);
    }

    pub fn total_capacity(&self) -> u32 {
        self.total_sectors * 512 // Assuming sector size is 512 bytes
    }

    pub fn free_capacity(&self) -> u32 {
        self.free_sectors * 512 // Assuming sector size is 512 bytes
    }

    pub fn cluster_size(&self) -> u32 {
        (self.sectors_per_cluster as u32) * 512 // Assuming sector size is 512 bytes
    }
}
