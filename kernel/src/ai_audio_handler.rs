extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AudioBuffer {
    data: Vec<u8>,
    sample_rate: u32,
    channels: u16,
}

impl AudioBuffer {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        AudioBuffer {
            data: Vec::new(),
            sample_rate,
            channels,
        }
    }

    pub fn add_sample(&mut self, sample: u8) {
        self.data.push(sample);
    }

    pub fn get_samples(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_buffer(&mut self) {
        self.data.clear();
    }

    pub fn buffer_size(&self) -> usize {
        self.data.len()
    }
}

pub extern "C" fn create_audio_buffer(sample_rate: u32, channels: u16) -> *mut AudioBuffer {
    Box::into_raw(Box::new(AudioBuffer::new(sample_rate, channels)))
}

pub extern "C" fn add_sample_to_buffer(buffer: *mut AudioBuffer, sample: u8) {
    if let Some(buf) = unsafe { buffer.as_mut() } {
        buf.add_sample(sample);
    }
}

pub extern "C" fn get_samples_from_buffer(buffer: *const AudioBuffer) -> *const u8 {
    if let Some(buf) = unsafe { buffer.as_ref() } {
        buf.get_samples().as_ptr()
    } else {
        core::ptr::null()
    }
}

pub extern "C" fn clear_audio_buffer(buffer: *mut AudioBuffer) {
    if let Some(buf) = unsafe { buffer.as_mut() } {
        buf.clear_buffer();
    }
}

pub extern "C" fn get_buffer_size(buffer: *const AudioBuffer) -> usize {
    if let Some(buf) = unsafe { buffer.as_ref() } {
        buf.buffer_size()
    } else {
        0
    }
}
