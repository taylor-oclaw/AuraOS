//! Aura Compositor
//! The heart of AuraOS's visual experience.
//! 
//! Unlike traditional window managers, the Aura Compositor manages "Surfaces"
//! — fluid, context-aware visual regions that adapt to the user's activity.
//! 
//! No title bars. No window chrome. No minimize/maximize/close buttons.
//! Surfaces flow, merge, split, and dissolve based on intent.

pub mod scene;
pub mod animation;
pub mod layout;
pub mod blur;
pub mod render;

/// A Surface is the fundamental visual unit in AuraOS
/// (replaces "window" from traditional OS)
#[derive(Debug, Clone)]
pub struct Surface {
    pub id: SurfaceId,
    pub kind: SurfaceKind,
    pub bounds: Rect,
    pub opacity: f32,           // 0.0 = invisible, 1.0 = fully visible
    pub blur_radius: f32,       // Background blur (glassmorphism)
    pub corner_radius: f32,     // Rounded corners
    pub z_order: i32,
    pub focused: bool,
    pub visible: bool,
    pub content: SurfaceContent,
    pub animation: Option<Animation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceKind {
    /// Full-screen immersive surface (e.g., media, reading)
    Immersive,
    /// Floating surface with glassmorphism (e.g., quick notes, chat)
    Floating,
    /// Panel surface anchored to screen edge (e.g., notification center)
    Panel,
    /// Ambient surface that provides background context (e.g., weather, time)
    Ambient,
    /// Overlay surface for transient interactions (e.g., command palette)
    Overlay,
    /// System surface (e.g., setup wizard, lock screen)
    System,
}

/// What's inside a Surface
#[derive(Debug, Clone)]
pub enum SurfaceContent {
    /// GPU-rendered content (most surfaces)
    Rendered,
    /// Video/camera stream
    MediaStream,
    /// Web content (embedded browser)
    WebView,
    /// Terminal/text content
    Terminal,
}

/// Rectangle in screen coordinates
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Animation state
#[derive(Debug, Clone)]
pub struct Animation {
    pub kind: AnimationKind,
    pub progress: f32,     // 0.0 to 1.0
    pub duration_ms: u32,
    pub easing: Easing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationKind {
    FadeIn,
    FadeOut,
    SlideUp,
    SlideDown,
    SlideLeft,
    SlideRight,
    Scale,
    Morph,        // Surface transforms shape
    Dissolve,     // Surface breaks into particles
    Materialize,  // Surface assembles from particles
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Spring,       // Bouncy, natural feel
    Smooth,       // Bezier curve
}
