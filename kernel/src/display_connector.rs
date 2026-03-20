extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DisplayConnector {
    connected: bool,
    resolution: (u32, u32),
    refresh_rate: u32,
    supported_resolutions: Vec<(u32, u32)>,
}

impl DisplayConnector {
    pub fn new(resolution: (u32, u32), refresh_rate: u32) -> Self {
        let mut supported_resolutions = Vec::new();
        supported_resolutions.push((1920, 1080));
        supported_resolutions.push((2560, 1440));
        supported_resolutions.push((3840, 2160));

        DisplayConnector {
            connected: true,
            resolution,
            refresh_rate,
            supported_resolutions,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn set_resolution(&mut self, resolution: (u32, u32)) -> Result<(), String> {
        if self.supported_resolutions.contains(&resolution) {
            self.resolution = resolution;
            Ok(())
        } else {
            Err(String::from("Unsupported resolution"))
        }
    }

    pub fn get_resolution(&self) -> (u32, u32) {
        self.resolution
    }

    pub fn set_refresh_rate(&mut self, refresh_rate: u32) -> Result<(), String> {
        if refresh_rate >= 60 && refresh_rate <= 144 {
            self.refresh_rate = refresh_rate;
            Ok(())
        } else {
            Err(String::from("Invalid refresh rate"))
        }
    }

    pub fn get_refresh_rate(&self) -> u32 {
        self.refresh_rate
    }

    pub fn list_supported_resolutions(&self) -> Vec<(u32, u32)> {
        self.supported_resolutions.clone()
    }
}
