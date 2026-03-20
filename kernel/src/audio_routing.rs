extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub enum AudioDevice {
    Speaker,
    Headphones,
    Bluetooth(String),
    Hdmi,
    Usb(String)
}

pub struct AudioRoute {
    pub source: String,
    pub destination: String,
    pub volume: u8,
    pub active: bool
}

pub struct AudioRouter {
    pub routes: Vec<AudioRoute>,
    pub devices: Vec<AudioDevice>,
    pub master_volume: u8,
    pub default_output: usize
}

impl AudioRouter {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            devices: vec![AudioDevice::Speaker],
            master_volume: 75,
            default_output: 0
        }
    }

    pub fn add_device(&mut self, device: AudioDevice) {
        self.devices.push(device);
    }

    pub fn route(&mut self, source: &str, dest_idx: usize, volume: u8) {
        if let Some(device) = self.devices.get(dest_idx) {
            self.routes.push(AudioRoute {
                source: String::from(source),
                destination: String::from("default"),
                volume,
                active: true
            });
        }
    }

    pub fn set_default_output(&mut self, idx: usize) {
        if idx < self.devices.len() {
            self.default_output = idx;
        }
    }

    pub fn set_master_volume(&mut self, vol: u8) {
        self.master_volume = if vol > 100 { 100 } else { vol };
    }

    pub fn mute_source(&mut self, source: &str) {
        for r in &mut self.routes {
            if r.source == source {
                r.active = false;
            }
        }
    }

    pub fn active_routes(&self) -> Vec<&AudioRoute> {
        self.routes.iter().filter(|r| r.active).collect()
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}
