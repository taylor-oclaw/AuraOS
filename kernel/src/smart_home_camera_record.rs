extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct SmartHomeCameraRecord {
    name: String,
    resolution: (u32, u32),
    fps: u32,
    storage_capacity: u64,
    recordings: Vec<String>,
}

impl SmartHomeCameraRecord {
    pub fn new(name: &str, resolution: (u32, u32), fps: u32, storage_capacity: u64) -> Self {
        SmartHomeCameraRecord {
            name: String::from(name),
            resolution,
            fps,
            storage_capacity,
            recordings: Vec::new(),
        }
    }

    pub fn record(&mut self, recording_name: &str) -> bool {
        if self.storage_capacity > 0 {
            self.recordings.push(String::from(recording_name));
            true
        } else {
            false
        }
    }

    pub fn delete_recording(&mut self, recording_name: &str) -> bool {
        let index = self.recordings.iter().position(|r| r == recording_name);
        if let Some(i) = index {
            self.recordings.remove(i);
            true
        } else {
            false
        }
    }

    pub fn list_recordings(&self) -> Vec<String> {
        self.recordings.clone()
    }

    pub fn get_resolution(&self) -> (u32, u32) {
        self.resolution
    }

    pub fn set_resolution(&mut self, resolution: (u32, u32)) {
        self.resolution = resolution;
    }
}
