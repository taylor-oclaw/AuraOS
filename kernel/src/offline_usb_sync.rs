extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OfflineUsbSync {
    device_id: String,
    sync_status: bool,
    last_sync_time: u64,
    files_to_sync: Vec<String>,
    error_log: Vec<String>,
}

impl OfflineUsbSync {
    pub fn new(device_id: &str) -> Self {
        OfflineUsbSync {
            device_id: String::from(device_id),
            sync_status: false,
            last_sync_time: 0,
            files_to_sync: Vec::new(),
            error_log: Vec::new(),
        }
    }

    pub fn add_file_to_sync(&mut self, file_path: &str) {
        self.files_to_sync.push(String::from(file_path));
    }

    pub fn remove_file_from_sync(&mut self, file_path: &str) {
        if let Some(index) = self.files_to_sync.iter().position(|x| x == file_path) {
            self.files_to_sync.remove(index);
        }
    }

    pub fn start_sync(&mut self) -> bool {
        // Simulate sync process
        self.sync_status = true;
        self.last_sync_time = 1633072800; // Example timestamp
        self.error_log.clear();
        true
    }

    pub fn stop_sync(&mut self) {
        self.sync_status = false;
    }

    pub fn get_last_sync_time(&self) -> u64 {
        self.last_sync_time
    }

    pub fn log_error(&mut self, error_message: &str) {
        self.error_log.push(String::from(error_message));
    }

    pub fn get_error_log(&self) -> Vec<String> {
        self.error_log.clone()
    }
}
