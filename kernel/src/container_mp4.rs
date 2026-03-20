extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn container_mp4_init() {
    // Initialization code for the module
}

pub extern "C" fn container_mp4_exit() {
    // Cleanup code for the module
}

pub struct ContainerMP4 {
    name: String,
    tracks: Vec<String>,
    duration: u32, // in seconds
    resolution: (u32, u32),
    bitrate: u32, // in kbps
}

impl ContainerMP4 {
    pub fn new(name: &str) -> Self {
        ContainerMP4 {
            name: String::from(name),
            tracks: Vec::new(),
            duration: 0,
            resolution: (1920, 1080), // default resolution
            bitrate: 5000, // default bitrate in kbps
        }
    }

    pub fn add_track(&mut self, track_name: &str) {
        self.tracks.push(String::from(track_name));
    }

    pub fn get_tracks(&self) -> &[String] {
        &self.tracks
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    pub fn get_duration(&self) -> u32 {
        self.duration
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.resolution = (width, height);
    }

    pub fn get_resolution(&self) -> (u32, u32) {
        self.resolution
    }

    pub fn set_bitrate(&mut self, bitrate: u32) {
        self.bitrate = bitrate;
    }

    pub fn get_bitrate(&self) -> u32 {
        self.bitrate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_mp4() {
        let mut container = ContainerMP4::new("Test Video");
        assert_eq!(container.get_name(), "Test Video");

        container.add_track("Audio Track");
        container.add_track("Video Track");
        assert_eq!(container.get_tracks().len(), 2);

        container.set_duration(120);
        assert_eq!(container.get_duration(), 120);

        container.set_resolution(3840, 2160);
        assert_eq!(container.get_resolution(), (3840, 2160));

        container.set_bitrate(15000);
        assert_eq!(container.get_bitrate(), 15000);
    }
}
