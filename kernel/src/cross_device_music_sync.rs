extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut music_sync = CrossDeviceMusicSync::new();
    music_sync.add_device("device1");
    music_sync.add_device("device2");
    music_sync.sync_music("song1", "device1");
    music_sync.sync_music("song2", "device2");
    music_sync.remove_device("device1");
}

pub struct CrossDeviceMusicSync {
    devices: Vec<String>,
    music_library: Vec<(String, String)>, // (song_name, device)
}

impl CrossDeviceMusicSync {
    pub fn new() -> Self {
        CrossDeviceMusicSync {
            devices: Vec::new(),
            music_library: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        if !self.devices.contains(&device_name.to_string()) {
            self.devices.push(device_name.to_string());
        }
    }

    pub fn remove_device(&mut self, device_name: &str) {
        self.devices.retain(|d| d != device_name);
        self.music_library.retain(|(_, dev)| dev != device_name);
    }

    pub fn sync_music(&mut self, song_name: &str, device_name: &str) -> bool {
        if self.devices.contains(&device_name.to_string()) {
            self.music_library.push((song_name.to_string(), device_name.to_string()));
            true
        } else {
            false
        }
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn get_device_music(&self, device_name: &str) -> Vec<String> {
        self.music_library
            .iter()
            .filter_map(|(song, dev)| if dev == device_name { Some(song.clone()) } else { None })
            .collect()
    }
}
