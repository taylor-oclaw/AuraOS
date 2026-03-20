extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DiskPartition {
    name: String,
    size: u64,
    start_sector: u64,
    end_sector: u64,
    file_system_type: String,
}

impl DiskPartition {
    pub fn new(name: &str, size: u64, start_sector: u64, end_sector: u64, file_system_type: &str) -> Self {
        DiskPartition {
            name: String::from(name),
            size,
            start_sector,
            end_sector,
            file_system_type: String::from(file_system_type),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_start_sector(&self) -> u64 {
        self.start_sector
    }

    pub fn get_end_sector(&self) -> u64 {
        self.end_sector
    }

    pub fn get_file_system_type(&self) -> &str {
        &self.file_system_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_partition() {
        let partition = DiskPartition::new("partition1", 1024, 2048, 3072, "ext4");
        assert_eq!(partition.get_name(), "partition1");
        assert_eq!(partition.get_size(), 1024);
        assert_eq!(partition.get_start_sector(), 2048);
        assert_eq!(partition.get_end_sector(), 3072);
        assert_eq!(partition.get_file_system_type(), "ext4");
    }
}
