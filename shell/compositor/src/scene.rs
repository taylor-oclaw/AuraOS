//! Scene graph for the compositor
//! Manages the spatial relationship between all surfaces.

use super::{Surface, SurfaceId, Rect};

/// The scene graph containing all visible surfaces
pub struct Scene {
    surfaces: Vec<Surface>,
    screen_width: u32,
    screen_height: u32,
    /// The ambient background color/gradient
    background: Background,
}

#[derive(Debug, Clone)]
pub enum Background {
    /// Solid color
    Solid { r: u8, g: u8, b: u8 },
    /// Linear gradient
    Gradient { 
        from: (u8, u8, u8),
        to: (u8, u8, u8),
        angle_deg: f32,
    },
    /// Dynamic wallpaper that shifts with time of day
    Dynamic {
        dawn: (u8, u8, u8),     // Warm sunrise
        day: (u8, u8, u8),      // Clear sky
        dusk: (u8, u8, u8),     // Amber sunset
        night: (u8, u8, u8),    // Deep blue
    },
    /// Blurred live view (camera/screen content)
    LiveBlur { blur_radius: f32 },
}

impl Default for Background {
    fn default() -> Self {
        // AuraOS signature: deep space gradient
        Background::Gradient {
            from: (10, 10, 30),     // Deep navy
            to: (30, 15, 60),       // Dark purple
            angle_deg: 135.0,
        }
    }
}

impl Scene {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            surfaces: Vec::new(),
            screen_width: width,
            screen_height: height,
            background: Background::default(),
        }
    }

    pub fn add_surface(&mut self, surface: Surface) {
        self.surfaces.push(surface);
        self.sort_by_z();
    }

    pub fn remove_surface(&mut self, id: SurfaceId) {
        self.surfaces.retain(|s| s.id != id);
    }

    pub fn surface_at(&self, x: f32, y: f32) -> Option<&Surface> {
        // Iterate in reverse z-order (topmost first)
        self.surfaces.iter().rev().find(|s| {
            s.visible
                && x >= s.bounds.x
                && x <= s.bounds.x + s.bounds.width
                && y >= s.bounds.y
                && y <= s.bounds.y + s.bounds.height
        })
    }

    fn sort_by_z(&mut self) {
        self.surfaces.sort_by_key(|s| s.z_order);
    }

    pub fn surfaces(&self) -> &[Surface] {
        &self.surfaces
    }
}
