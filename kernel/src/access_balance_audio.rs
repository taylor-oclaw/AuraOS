extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

struct AudioDevice {
    name: String,
    volume: u8,
    is_muted: bool,
    supported_formats: Vec<String>,
}

impl AudioDevice {
    pub fn new(name: &str, volume: u8) -> Self {
        AudioDevice {
            name: String::from(name),
            volume,
            is_muted: false,
            supported_formats: vec![String::from("PCM"), String::from("MP3")],
        }
    }

    pub fn get_name(&self) -> &str {
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

    pub fn get_supported_formats(&self) -> &Vec<String> {
        &self.supported_formats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_device() {
        let mut device = AudioDevice::new("Test Device", 50);

        assert_eq!(device.get_name(), "Test Device");
        assert_eq!(device.get_volume(), 50);
        assert!(!device.is_muted());

        device.set_volume(75);
        assert_eq!(device.get_volume(), 75);

        device.mute();
        assert!(device.is_muted());

        device.unmute();
        assert!(!device.is_muted());

        let formats = device.get_supported_formats();
        assert_eq!(formats.len(), 2);
        assert!(formats.contains(&String::from("PCM")));
        assert!(formats.contains(&String::from("MP3")));
    }
}
