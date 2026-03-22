extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct BackupUsbMirrorFiltered {
    device_path: String,
    file_system_type: u8,
}

impl BackupUsbMirrorFiltered {
    pub fn new(device_path: &str, file_system_type: u8) -> Self {
        BackupUsbMirrorFiltered {
            device_path: device_path.to_string(),
            file_system_type,
        }
    }

    pub fn get_device_path(&self) -> &str {
        self.device_path.as_str()
    }

    pub fn set_file_system_type(&mut self, new_type: u8) {
        self.file_system_type = new_type;
    }

    pub fn is_valid(&self) -> bool {
        // Logic to check if the device path and file system type are valid
        true
    }

    pub fn backup_device(&self) -> Result<(), ()> {
        // Logic to backup the device
        Ok(())
    }
}

pub struct BackupUsbMirrorFilteredResult {
    status: u8,
    message: String,
}

impl BackupUsbMirrorFilteredResult {
    pub fn new(status: u8, message: &str) -> Self {
        BackupUsbMirrorFilteredResult {
            status,
            message: message.to_string(),
        }
    }

    pub fn get_status(&self) -> u8 {
        self.status
    }

    pub fn get_message(&self) -> &str {
        self.message.as_str()
    }
}