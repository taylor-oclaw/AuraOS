extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DisplayPortHandler {
    connected: bool,
    resolution: (u32, u32),
    refresh_rate: f64,
    supported_resolutions: Vec<(u32, u32)>,
    current_mode: String,
}

impl DisplayPortHandler {
    pub fn new() -> Self {
        DisplayPortHandler {
            connected: false,
            resolution: (1920, 1080),
            refresh_rate: 60.0,
            supported_resolutions: vec![(1920, 1080), (2560, 1440), (3840, 2160)],
            current_mode: String::from("1920x1080@60Hz"),
        }
    }

    pub fn connect(&mut self) {
        self.connected = true;
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) -> bool {
        if self.supported_resolutions.contains(&(width, height)) {
            self.resolution = (width, height);
            true
        } else {
            false
        }
    }

    pub fn get_resolution(&self) -> (u32, u32) {
        self.resolution
    }

    pub fn set_refresh_rate(&mut self, rate: f64) -> bool {
        if rate > 0.0 {
            self.refresh_rate = rate;
            true
        } else {
            false
        }
    }

    pub fn get_refresh_rate(&self) -> f64 {
        self.refresh_rate
    }

    pub fn list_supported_resolutions(&self) -> Vec<(u32, u32)> {
        self.supported_resolutions.clone()
    }

    pub fn set_mode(&mut self, mode: &str) -> bool {
        if self.supported_modes().contains(&mode.to_string()) {
            self.current_mode = String::from(mode);
            true
        } else {
            false
        }
    }

    pub fn get_current_mode(&self) -> String {
        self.current_mode.clone()
    }

    fn supported_modes(&self) -> Vec<String> {
        self.supported_resolutions.iter().map(|&(w, h)| format!("{}x{}@{}Hz", w, h, self.refresh_rate)).collect()
    }
}
