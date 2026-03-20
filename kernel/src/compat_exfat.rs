extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct ExFatFileSystem {
    volume_label: String,
    total_sectors: u64,
    free_sectors: u64,
    cluster_size: u32,
    root_directory_cluster: u32,
}

impl ExFatFileSystem {
    pub fn new(volume_label: &str, total_sectors: u64, free_sectors: u64, cluster_size: u32, root_directory_cluster: u32) -> Self {
        ExFatFileSystem {
            volume_label: String::from(volume_label),
            total_sectors,
            free_sectors,
            cluster_size,
            root_directory_cluster,
        }
    }

    pub fn get_volume_label(&self) -> &str {
        &self.volume_label
    }

    pub fn get_total_sectors(&self) -> u64 {
        self.total_sectors
    }

    pub fn get_free_sectors(&self) -> u64 {
        self.free_sectors
    }

    pub fn get_cluster_size(&self) -> u32 {
        self.cluster_size
    }

    pub fn get_root_directory_cluster(&self) -> u32 {
        self.root_directory_cluster
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exfat_filesystem() {
        let exfat = ExFatFileSystem::new("MYVOLUME", 10000, 5000, 4096, 2);
        assert_eq!(exfat.get_volume_label(), "MYVOLUME");
        assert_eq!(exfat.get_total_sectors(), 10000);
        assert_eq!(exfat.get_free_sectors(), 5000);
        assert_eq!(exfat.get_cluster_size(), 4096);
        assert_eq!(exfat.get_root_directory_cluster(), 2);
    }
}
