extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Cleanup the module
}

struct AudioDevice {
    name: String,
    volume: u8,
    is_muted: bool,
}

impl AudioDevice {
    pub fn new(name: &str, volume: u8) -> Self {
        AudioDevice {
            name: String::from(name),
            volume,
            is_muted: false,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_volume(&mut self, volume: u8) {
        if volume <= 100 {
            self.volume = volume;
        } else {
            self.volume = 100;
        }
    }

    pub fn get_volume(&self) -> u8 {
        self.volume
    }

    pub fn mute(&mut self) {
        self.is_muted = true;
    }

    pub fn unmute(&mut self) {
        self.is_muted = false;
    }

    pub fn is_muted(&self) -> bool {
        self.is_muted
    }
}

struct AudioManager {
    devices: Vec<AudioDevice>,
}

impl AudioManager {
    pub fn new() -> Self {
        AudioManager {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: AudioDevice) {
        self.devices.push(device);
    }

    pub fn get_device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn get_device_by_name(&self, name: &str) -> Option<&AudioDevice> {
        self.devices.iter().find(|&dev| dev.get_name() == name)
    }

    pub fn set_device_volume(&mut self, name: &str, volume: u8) {
        if let Some(device) = self.devices.iter_mut().find(|dev| dev.get_name() == name) {
            device.set_volume(volume);
        }
    }

    pub fn mute_device(&mut self, name: &str) {
        if let Some(device) = self.devices.iter_mut().find(|dev| dev.get_name() == name) {
            device.mute();
        }
    }

    pub fn unmute_device(&mut self, name: &str) {
        if let Some(device) = self.devices.iter_mut().find(|dev| dev.get_name() == name) {
            device.unmute();
        }
    }
}

#[no_mangle]
pub extern "C" fn aura_audio_mgr_init() -> *mut AudioManager {
    Box::into_raw(Box::new(AudioManager::new()))
}

#[no_mangle]
pub extern "C" fn aura_audio_mgr_add_device(mgr: *mut AudioManager, name: &str, volume: u8) {
    unsafe {
        (*mgr).add_device(AudioDevice::new(name, volume));
    }
}

#[no_mangle]
pub extern "C" fn aura_audio_mgr_get_device_count(mgr: *const AudioManager) -> usize {
    unsafe { (*mgr).get_device_count() }
}

#[no_mangle]
pub extern "C" fn aura_audio_mgr_set_device_volume(mgr: *mut AudioManager, name: &str, volume: u8) {
    unsafe {
        (*mgr).set_device_volume(name, volume);
    }
}

#[no_mangle]
pub extern "C" fn aura_audio_mgr_mute_device(mgr: *mut AudioManager, name: &str) {
    unsafe {
        (*mgr).mute_device(name);
    }
}

#[no_mangle]
pub extern "C" fn aura_audio_mgr_unmute_device(mgr: *mut AudioManager, name: &str) {
    unsafe {
        (*mgr).unmute_device(name);
    }
}
