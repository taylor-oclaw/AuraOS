extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AdversarialImage {
    data: Vec<u8>,
    width: usize,
    height: usize,
    channels: usize,
}

impl AdversarialImage {
    pub fn new(width: usize, height: usize, channels: usize) -> Self {
        let size = width * height * channels;
        AdversarialImage {
            data: vec![0; size],
            width,
            height,
            channels,
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize, channel: usize) -> u8 {
        if x < self.width && y < self.height && channel < self.channels {
            let index = (y * self.width + x) * self.channels + channel;
            self.data[index]
        } else {
            0
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, channel: usize, value: u8) {
        if x < self.width && y < self.height && channel < self.channels {
            let index = (y * self.width + x) * self.channels + channel;
            self.data[index] = value;
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_channels(&self) -> usize {
        self.channels
    }

    pub fn flip_horizontal(&mut self) {
        for y in 0..self.height {
            for x in 0..(self.width / 2) {
                for c in 0..self.channels {
                    let left_index = (y * self.width + x) * self.channels + c;
                    let right_index = (y * self.width + (self.width - x - 1)) * self.channels + c;
                    self.data.swap(left_index, right_index);
                }
            }
        }
    }

    pub fn flip_vertical(&mut self) {
        for y in 0..(self.height / 2) {
            for x in 0..self.width {
                for c in 0..self.channels {
                    let top_index = (y * self.width + x) * self.channels + c;
                    let bottom_index = ((self.height - y - 1) * self.width + x) * self.channels + c;
                    self.data.swap(top_index, bottom_index);
                }
            }
        }
    }

    pub fn to_grayscale(&mut self) {
        if self.channels != 3 {
            return;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) * self.channels;
                let r = self.data[index] as f32;
                let g = self.data[index + 1] as f32;
                let b = self.data[index + 2] as f32;
                let gray = ((r * 0.299) + (g * 0.587) + (b * 0.114)) as u8;
                self.data[index] = gray;
                self.data[index + 1] = gray;
                self.data[index + 2] = gray;
            }
        }

        self.channels = 1;
    }
}
