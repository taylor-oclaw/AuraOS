extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct AudioCodecFLAC {
    sample_rate: u32,
    channels: u16,
    bits_per_sample: u16,
    data: Vec<u8>,
}

impl AudioCodecFLAC {
    pub fn new(sample_rate: u32, channels: u16, bits_per_sample: u16) -> Self {
        AudioCodecFLAC {
            sample_rate,
            channels,
            bits_per_sample,
            data: Vec::new(),
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn set_channels(&mut self, channels: u16) {
        self.channels = channels;
    }

    pub fn get_channels(&self) -> u16 {
        self.channels
    }

    pub fn set_bits_per_sample(&mut self, bits_per_sample: u16) {
        self.bits_per_sample = bits_per_sample;
    }

    pub fn get_bits_per_sample(&self) -> u16 {
        self.bits_per_sample
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}
