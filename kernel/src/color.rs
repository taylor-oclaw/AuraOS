extern crate alloc;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0, a: 255 };
    pub const RED: Self = Self { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255, a: 255 };
    pub const CYAN: Self = Self { r: 0, g: 255, b: 255, a: 255 };
    pub const YELLOW: Self = Self { r: 255, g: 255, b: 0, a: 255 };

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color { r, g, b, a: 255 }
    }

    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn blend(&self, other: &Color, factor: f32) -> Color {
        let r = (self.r as f32 * (1.0 - factor) + other.r as f32 * factor).clamp(0.0, 255.0) as u8;
        let g = (self.g as f32 * (1.0 - factor) + other.g as f32 * factor).clamp(0.0, 255.0) as u8;
        let b = (self.b as f32 * (1.0 - factor) + other.b as f32 * factor).clamp(0.0, 255.0) as u8;
        let a = (self.a as f32 * (1.0 - factor) + other.a as f32 * factor).clamp(0.0, 255.0) as u8;
        Color { r, g, b, a }
    }

    pub fn lighten(&self, amount: f32) -> Color {
        let r = (self.r as f32 + (255.0 - self.r as f32) * amount).clamp(0.0, 255.0) as u8;
        let g = (self.g as f32 + (255.0 - self.g as f32) * amount).clamp(0.0, 255.0) as u8;
        let b = (self.b as f32 + (255.0 - self.b as f32) * amount).clamp(0.0, 255.0) as u8;
        Color { r, g, b, a: self.a }
    }

    pub fn darken(&self, amount: f32) -> Color {
        let r = (self.r as f32 * (1.0 - amount)).clamp(0.0, 255.0) as u8;
        let g = (self.g as f32 * (1.0 - amount)).clamp(0.0, 255.0) as u8;
        let b = (self.b as f32 * (1.0 - amount)).clamp(0.0, 255.0) as u8;
        Color { r, g, b, a: self.a }
    }
}