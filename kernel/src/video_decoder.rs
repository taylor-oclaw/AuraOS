extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum VideoCodec {
    H264,
    H265,
    Vp9,
    Av1,
    Raw,
}

pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub pts_ms: u64,
    pub keyframe: bool,
}

pub struct VideoDecoder {
    pub codec: VideoCodec,
    pub width: u32,
    pub height: u32,
    pub fps: u8,
    pub frames_decoded: u64,
    pub buffer: Vec<VideoFrame>,
    pub max_buffer: usize,
}

impl VideoDecoder {
    pub fn new(codec: VideoCodec, w: u32, h: u32, fps: u8) -> Self {
        Self {
            codec,
            width: w,
            height: h,
            fps,
            frames_decoded: 0,
            buffer: Vec::new(),
            max_buffer: 30,
        }
    }

    pub fn decode_frame(&mut self, data: Vec<u8>, pts_ms: u64, keyframe: bool) {
        self.buffer.push(VideoFrame {
            width: self.width,
            height: self.height,
            data,
            pts_ms,
            keyframe,
        };
        self.frames_decoded += 1;
        if self.buffer.len() > self.max_buffer {
            self.buffer.remove(0);
        }
    }

    pub fn next_frame(&mut self) -> Option<VideoFrame> {
        if self.buffer.is_empty() {
            None
        } else {
            Some(self.buffer.remove(0))
        }
    }

    pub fn buffered_frames(&self) -> usize {
        self.buffer.len()
    }

    pub fn seek(&mut self) {
        self.buffer.clear();
    }
)}
