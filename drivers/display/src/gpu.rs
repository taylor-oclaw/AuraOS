//! GPU abstraction layer
//! Provides a unified interface for GPU-accelerated rendering.
//! Backend: wgpu (Vulkan/Metal/DX12/OpenGL)

/// GPU device capabilities
#[derive(Debug, Clone)]
pub struct GpuCapabilities {
    pub name: String,
    pub vendor: GpuVendor,
    pub vram_mb: u32,
    pub vulkan: bool,
    pub compute: bool,
    pub ray_tracing: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuVendor {
    Intel,
    AMD,
    Nvidia,
    Apple,
    Qualcomm,
    Virtual,    // QEMU virtio-gpu
    Unknown,
}
