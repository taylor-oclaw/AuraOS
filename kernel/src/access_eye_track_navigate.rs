extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("Access Eye Track Navigate module loaded!");
    0
}

pub struct AccessEyeTrackNavigate {
    eye_data: Vec<u8>,
    track_data: Vec<u8>,
    navigate_data: Vec<u8>,
    status: String,
}

impl AccessEyeTrackNavigate {
    pub fn new() -> Self {
        AccessEyeTrackNavigate {
            eye_data: Vec::new(),
            track_data: Vec::new(),
            navigate_data: Vec::new(),
            status: String::from("Initialized"),
        }
    }

    pub fn update_eye_data(&mut self, data: &[u8]) {
        self.eye_data.clear();
        self.eye_data.extend_from_slice(data);
        self.status = String::from("Eye Data Updated");
    }

    pub fn get_eye_data(&self) -> &[u8] {
        &self.eye_data
    }

    pub fn update_track_data(&mut self, data: &[u8]) {
        self.track_data.clear();
        self.track_data.extend_from_slice(data);
        self.status = String::from("Track Data Updated");
    }

    pub fn get_track_data(&self) -> &[u8] {
        &self.track_data
    }

    pub fn update_navigate_data(&mut self, data: &[u8]) {
        self.navigate_data.clear();
        self.navigate_data.extend_from_slice(data);
        self.status = String::from("Navigate Data Updated");
    }

    pub fn get_navigate_data(&self) -> &[u8] {
        &self.navigate_data
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}
