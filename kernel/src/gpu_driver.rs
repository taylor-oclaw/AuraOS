extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum GpuVendor {
    Intel,
    Amd,
    Nvidia,
    VirtIO,
    Unknown,
}

pub struct GpuInfo {
    pub vendor: GpuVendor,
    pub name: String,
    pub vram_mb: u32,
    pub max_resolution_w: u32,
    pub max_resolution_h: u32,
    pub supports_3d: bool,
}

pub struct GpuBuffer {
    pub id: u64,
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub data_offset: u64,
}

pub struct GpuDriver {
    pub info: GpuInfo,
    pub buffers: Vec<GpuBuffer>,
    pub next_buffer_id: u64,
    pub initialized: bool,
    pub current_mode_w: u32,
    pub current_mode_h: u32,
}

impl GpuDriver {
    pub fn new() -> Self {
        Self {
            info: GpuInfo {
                vendor: GpuVendor::VirtIO,
                name: String::from("VirtIO GPU"),
                vram_mb: 256,
                max_resolution_w: 3840,
                max_resolution_h: 2160,
                supports_3d: false,
            },
            buffers: Vec::new(),
            next_buffer_id: 1,
            initialized: false,
            current_mode_w: 1920,
            current_mode_h: 1080,
        }
    }

    pub fn init(&mut self) -> bool {
        self.initialized = true;
        true
    }

    pub fn set_mode(&mut self, w: u32, h: u32) -> bool {
        if w <= self.info.max_resolution_w && h <= self.info.max_resolution_h {
            self.current_mode_w = w;
            self.current_mode_h = h;
            true
        } else {
            false
        }
    }

    pub fn create_buffer(&mut self, w: u32, h: u32) -> u64 {
        let id = self.next_buffer_id;
        self.next_buffer_id += 1;
        self.buffers.push(GpuBuffer {
            id,
            width: w,
            height: h,
            format: 32,
            data_offset: 0,
        };
        id
    }

    pub fn destroy_buffer(&mut self, id: u64) {
        self.buffers.retain(|b| b.id != id);
    }

    pub fn buffer_count(&self) -> usize {
        self.buffers.len()
    }
)}
