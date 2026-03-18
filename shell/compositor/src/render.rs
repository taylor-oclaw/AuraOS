//! Render pipeline for the compositor
//! Orchestrates drawing of background, surfaces, blur, and overlays.

/// Render pass order
pub enum RenderPass {
    /// Draw background gradient/wallpaper
    Background,
    /// Draw ambient surfaces (weather, time, etc.)
    AmbientLayer,
    /// Apply background blur for glass surfaces
    BlurPass,
    /// Draw main surfaces
    SurfaceLayer,
    /// Draw floating/overlay surfaces
    OverlayLayer,
    /// Draw cursor and selection highlights
    CursorLayer,
    /// Draw system overlays (notifications, command palette)
    SystemLayer,
}

/// Frame timing for consistent 60fps+
pub struct FrameTiming {
    pub target_fps: u32,
    pub frame_budget_us: u64,
    pub last_frame_us: u64,
    pub frames_rendered: u64,
    pub dropped_frames: u64,
}

impl FrameTiming {
    pub fn new(target_fps: u32) -> Self {
        Self {
            target_fps,
            frame_budget_us: 1_000_000 / target_fps as u64,
            last_frame_us: 0,
            frames_rendered: 0,
            dropped_frames: 0,
        }
    }
}
