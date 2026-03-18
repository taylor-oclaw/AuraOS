//! AuraOS Display Driver
//! Supports framebuffer, GPU acceleration via wgpu, and multi-monitor.

pub mod framebuffer;
pub mod gpu;
pub mod modes;

/// Display output information
#[derive(Debug, Clone)]
pub struct DisplayOutput {
    pub id: u32,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub refresh_hz: u32,
    pub scale_factor: f32,
    pub primary: bool,
    pub connected: bool,
    pub connector: ConnectorType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorType {
    Internal,    // Laptop screen
    HDMI,
    DisplayPort,
    USB_C,
    VGA,
    DVI,
    Virtual,     // QEMU/VM
}

/// Pixel format for the framebuffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Bgr8,       // Blue-Green-Red, 8 bits each (common in BIOS/UEFI)
    Rgb8,       // Red-Green-Blue, 8 bits each
    Bgra8,      // With alpha
    Rgba8,
}
