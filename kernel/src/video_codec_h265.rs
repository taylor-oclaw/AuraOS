extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct VideoCodecH265 {
    width: u32,
    height: u32,
    frame_rate: f32,
    bit_rate: u32,
    resolution: String,
}

impl VideoCodecH265 {
    pub fn new(width: u32, height: u32, frame_rate: f32, bit_rate: u32) -> Self {
        let resolution = String::from("info");
        VideoCodecH265 {
            width,
            height,
            frame_rate,
            bit_rate,
            resolution,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_frame_rate(&self) -> f32 {
        self.frame_rate
    }

    pub fn get_bit_rate(&self) -> u32 {
        self.bit_rate
    }

    pub fn get_resolution(&self) -> &str {
        &self.resolution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_codec_h265() {
        let codec = VideoCodecH265::new(1920, 1080, 30.0, 5000);
        assert_eq!(codec.get_width(), 1920);
        assert_eq!(codec.get_height(), 1080);
        assert_eq!(codec.get_frame_rate(), 30.0);
        assert_eq!(codec.get_bit_rate(), 5000);
        assert_eq!(codec.get_resolution(), "1920x1080");
    }
}
