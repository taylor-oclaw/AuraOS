//! AuraOS Camera Driver
//! The camera is a first-class OS sensor, not just a peripheral.
//! 
//! Supported hardware:
//! - USB Video Class (UVC) cameras (most webcams)
//! - Intel RealSense (depth + RGB)
//! - IR cameras (face unlock)
//! - Phone cameras (via Aura Bridge)

pub mod uvc;
pub mod pipeline;

/// Camera device information
#[derive(Debug, Clone)]
pub struct CameraDevice {
    pub id: u32,
    pub name: String,
    pub kind: CameraKind,
    pub capabilities: CameraCapabilities,
    pub resolutions: Vec<Resolution>,
    pub connected: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraKind {
    RGB,            // Standard color camera
    IR,             // Infrared (face unlock)
    Depth,          // Depth sensing (RealSense, ToF)
    RGBDepth,       // Combined RGB + depth
    PhoneFront,     // Phone front camera (via Bridge)
    PhoneRear,      // Phone rear camera (via Bridge)
    Virtual,        // Virtual camera (software-generated)
}

#[derive(Debug, Clone, Copy)]
pub struct CameraCapabilities {
    pub autofocus: bool,
    pub hdr: bool,
    pub low_light: bool,
    pub optical_zoom: bool,
    pub stabilization: bool,
    pub depth_sensing: bool,
    pub ir_illuminator: bool,  // IR flood for face unlock
    pub wide_angle: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub format: PixelFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    YUYV,
    MJPEG,
    NV12,
    RGB24,
    RGBA32,
    Depth16,    // 16-bit depth map
    IR8,        // 8-bit infrared
}

/// Camera frame from any source
#[derive(Debug)]
pub struct CameraFrame {
    pub device_id: u32,
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    pub timestamp_ns: u64,
    pub data: Vec<u8>,
    pub metadata: FrameMetadata,
}

#[derive(Debug, Default)]
pub struct FrameMetadata {
    pub exposure_us: Option<u32>,
    pub gain: Option<f32>,
    pub white_balance_k: Option<u32>,
    pub focus_distance_mm: Option<u32>,
}
