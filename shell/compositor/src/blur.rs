//! Gaussian blur for glassmorphism effect
//! AuraOS's signature visual style: frosted glass surfaces.

/// Blur parameters
#[derive(Debug, Clone, Copy)]
pub struct BlurParams {
    pub radius: f32,           // Blur radius in pixels
    pub saturation: f32,       // Color saturation multiplier (1.0 = normal)
    pub brightness: f32,       // Brightness offset (-1.0 to 1.0)
    pub tint: (f32, f32, f32, f32), // RGBA tint overlay
}

impl Default for BlurParams {
    fn default() -> Self {
        // AuraOS default: subtle frosted glass
        Self {
            radius: 20.0,
            saturation: 1.2,
            brightness: 0.05,
            tint: (1.0, 1.0, 1.0, 0.1), // Very subtle white tint
        }
    }
}

/// The "dark glass" variant for dark mode
pub fn dark_glass() -> BlurParams {
    BlurParams {
        radius: 24.0,
        saturation: 1.4,
        brightness: -0.1,
        tint: (0.0, 0.0, 0.0, 0.3), // Dark tint
    }
}

/// The "light glass" variant for light mode
pub fn light_glass() -> BlurParams {
    BlurParams {
        radius: 16.0,
        saturation: 1.1,
        brightness: 0.15,
        tint: (1.0, 1.0, 1.0, 0.5), // White frosted
    }
}
