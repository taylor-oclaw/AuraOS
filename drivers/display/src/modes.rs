//! Display mode enumeration and management

/// A display mode (resolution + refresh rate)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_hz: u32,
    pub preferred: bool,
}

/// Common display modes
pub const COMMON_MODES: &[DisplayMode] = &[
    DisplayMode { width: 1920, height: 1080, refresh_hz: 60, preferred: true },
    DisplayMode { width: 2560, height: 1440, refresh_hz: 60, preferred: false },
    DisplayMode { width: 3840, height: 2160, refresh_hz: 60, preferred: false },
    DisplayMode { width: 1920, height: 1080, refresh_hz: 144, preferred: false },
    DisplayMode { width: 2560, height: 1440, refresh_hz: 144, preferred: false },
    DisplayMode { width: 1280, height: 720, refresh_hz: 60, preferred: false },
];
