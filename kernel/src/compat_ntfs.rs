extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct CompatNtfs {
    pub volume_name: String,
    pub file_system_type: String,
    pub total_sectors: u64,
    pub free_sectors: u64,
    pub sector_size: u32,
}

impl CompatNtfs {
    pub fn new(volume_name: &str, file_system_type: &str, total_sectors: u64, free_sectors: u64, sector_size: u32) -> Self {
        CompatNtfs {
            volume_name: String::from(volume_name),
            file_system_type: String::from(file_system_type),
            total_sectors,
            free_sectors,
            sector_size,
        }
    }

    pub fn get_volume_name(&self) -> &str {
        &self.volume_name
    }

    pub fn get_file_system_type(&self) -> &str {
        &self.file_system_type
    }

    pub fn get_total_capacity(&self) -> u64 {
        self.total_sectors * self.sector_size as u64
    }

    pub fn get_free_space(&self) -> u64 {
        self.free_sectors * self.sector_size as u64
    }

    pub fn calculate_used_space(&self) -> u64 {
        (self.total_sectors - self.free_sectors) * self.sector_size as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ntfs = CompatNtfs::new("Volume1", "NTFS", 2048, 512, 512);
        assert_eq!(ntfs.volume_name, "Volume1");
        assert_eq!(ntfs.file_system_type, "NTFS");
        assert_eq!(ntfs.total_sectors, 2048);
        assert_eq!(ntfs.free_sectors, 512);
        assert_eq!(ntfs.sector_size, 512);
    }

    #[test]
    fn test_get_volume_name() {
        let ntfs = CompatNtfs::new("Volume1", "NTFS", 2048, 512, 512);
        assert_eq!(ntfs.get_volume_name(), "Volume1");
    }

    #[test]
    fn test_get_file_system_type() {
        let ntfs = CompatNtfs::new("Volume1", "NTFS", 2048, 512, 512);
        assert_eq!(ntfs.get_file_system_type(), "NTFS");
    }

    #[test]
    fn test_get_total_capacity() {
        let ntfs = CompatNtfs::new("Volume1", "NTFS", 2048, 512, 512);
        assert_eq!(ntfs.get_total_capacity(), 1048576); // 2048 * 512
    }

    #[test]
    fn test_get_free_space() {
        let ntfs = CompatNtfs::new("Volume1", "NTFS", 2048, 512, 512);
        assert_eq!(ntfs.get_free_space(), 262144); // 512 * 512
    }

    #[test]
    fn test_calculate_used_space() {
        let ntfs = CompatNtfs::new("Volume1", "NTFS", 2048, 512, 512);
        assert_eq!(ntfs.calculate_used_space(), 786432); // (2048 - 512) * 512
    }
}
