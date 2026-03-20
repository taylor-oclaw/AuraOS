extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct VideoCodecVP9 {
    width: u32,
    height: u32,
    frame_rate: f32,
    bit_rate: u32,
    codec_version: String,
}

impl VideoCodecVP9 {
    pub fn new(width: u32, height: u32, frame_rate: f32, bit_rate: u32) -> Self {
        VideoCodecVP9 {
            width,
            height,
            frame_rate,
            bit_rate,
            codec_version: String::from("VP9"),
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn get_frame_rate(&self) -> f32 {
        self.frame_rate
    }

    pub fn set_frame_rate(&mut self, frame_rate: f32) {
        self.frame_rate = frame_rate;
    }

    pub fn get_bit_rate(&self) -> u32 {
        self.bit_rate
    }

    pub fn set_bit_rate(&mut self, bit_rate: u32) {
        self.bit_rate = bit_rate;
    }

    pub fn get_codec_version(&self) -> &str {
        &self.codec_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_codec_vp9() {
        let mut codec = VideoCodecVP9::new(1920, 1080, 60.0, 5000);

        assert_eq!(codec.get_width(), 1920);
        assert_eq!(codec.get_height(), 1080);
        assert_eq!(codec.get_frame_rate(), 60.0);
        assert_eq!(codec.get_bit_rate(), 5000);
        assert_eq!(codec.get_codec_version(), "VP9");

        codec.set_width(1280);
        codec.set_height(720);
        codec.set_frame_rate(30.0);
        codec.set_bit_rate(3000);

        assert_eq!(codec.get_width(), 1280);
        assert_eq!(codec.get_height(), 720);
        assert_eq!(codec.get_frame_rate(), 30.0);
        assert_eq!(codec.get_bit_rate(), 3000);
    }
}
