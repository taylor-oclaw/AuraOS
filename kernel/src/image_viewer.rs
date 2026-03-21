extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum ImageFormat {
    Png,
    Jpeg,
    Bmp,
    Gif,
    WebP,
    Raw
}

pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub pixels: Vec<u8>,
    pub path: String
}

pub struct ImageViewer {
    pub images: Vec<ImageData>,
    pub current_index: usize,
    pub zoom: f32,
    pub rotation: u16,
    pub slideshow: bool,
    pub slideshow_interval_ms: u64
}

impl ImageViewer {
    pub fn new() -> Self {
        Self {
            images: Vec::new(),
            current_index: 0,
            zoom: 1.0,
            rotation: 0,
            slideshow: false,
            slideshow_interval_ms: 3000
        }
    }

    pub fn open(&mut self, path: &str, w: u32, h: u32, format: ImageFormat, pixels: Vec<u8>) {
        self.images.push(ImageData {
            width: w,
            height: h,
            format,
            pixels,
            path: String::from(path)
        };
        self.current_index = self.images.len() - 1;
    }

    pub fn next(&mut self) {
        if !self.images.is_empty() {
            self.current_index = (self.current_index + 1) % self.images.len();
        }
    }

    pub fn prev(&mut self) {
        if !self.images.is_empty() {
            self.current_index = if self.current_index == 0 {
                self.images.len() - 1
            } else {
                self.current_index - 1
            };
        }
    }

    pub fn zoom_in(&mut self) {
        self.zoom *= 1.25;
        if self.zoom > 10.0 {
            self.zoom = 10.0;
        }
    }

    pub fn zoom_out(&mut self) {
        self.zoom *= 0.8;
        if self.zoom < 0.1 {
            self.zoom = 0.1;
        }
    }

    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 90) % 360;
    }

    pub fn current(&self) -> Option<&ImageData> {
        self.images.get(self.current_index)
    }
)}
