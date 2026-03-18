//! AuraOS Theme Engine
//! Defines the visual language of the operating system.
//! 
//! AuraOS ships with 3 themes:
//! - Nebula (default dark) — Deep space with luminous accents
//! - Aurora (light) — Ethereal northern lights
//! - Obsidian (OLED dark) — Pure black for OLED displays

/// Complete theme definition
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub mode: ThemeMode,
    pub colors: ColorPalette,
    pub typography: Typography,
    pub spacing: Spacing,
    pub radii: Radii,
    pub shadows: Shadows,
    pub blur: BlurStyle,
    pub animation: AnimationStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
    Oled,
    Auto,  // Adapts to ambient light
}

/// Color palette
#[derive(Debug, Clone)]
pub struct ColorPalette {
    // Backgrounds
    pub bg_primary: Color,
    pub bg_secondary: Color,
    pub bg_tertiary: Color,
    pub bg_surface: Color,       // Surface/card background
    pub bg_elevated: Color,      // Elevated surface

    // Foregrounds
    pub fg_primary: Color,
    pub fg_secondary: Color,
    pub fg_muted: Color,
    pub fg_inverse: Color,

    // Accents
    pub accent: Color,
    pub accent_hover: Color,
    pub accent_subtle: Color,

    // Semantic
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    // Special
    pub glow: Color,             // Luminous accent glow
    pub glass_tint: Color,       // Glassmorphism tint
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Debug, Clone)]
pub struct Typography {
    pub font_family: &'static str,
    pub font_size_xs: f32,
    pub font_size_sm: f32,
    pub font_size_md: f32,
    pub font_size_lg: f32,
    pub font_size_xl: f32,
    pub font_size_display: f32,
    pub line_height: f32,
    pub letter_spacing: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Spacing {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Radii {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub full: f32,   // Fully rounded (pill shape)
}

#[derive(Debug, Clone)]
pub struct Shadows {
    pub sm: Shadow,
    pub md: Shadow,
    pub lg: Shadow,
    pub glow: Shadow,
}

#[derive(Debug, Clone, Copy)]
pub struct Shadow {
    pub x: f32,
    pub y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct BlurStyle {
    pub surface: f32,      // Default surface blur
    pub elevated: f32,     // Elevated surface blur
    pub overlay: f32,      // Overlay blur
}

#[derive(Debug, Clone, Copy)]
pub struct AnimationStyle {
    pub duration_fast: u32,    // ms
    pub duration_normal: u32,
    pub duration_slow: u32,
}

// === BUILT-IN THEMES ===

/// Nebula — Default dark theme
/// Deep space with luminous cyan/purple accents
pub fn nebula() -> Theme {
    Theme {
        name: "Nebula",
        mode: ThemeMode::Dark,
        colors: ColorPalette {
            bg_primary: Color::rgb(8, 8, 24),
            bg_secondary: Color::rgb(14, 14, 36),
            bg_tertiary: Color::rgb(20, 20, 48),
            bg_surface: Color::rgba(24, 24, 56, 200),
            bg_elevated: Color::rgba(32, 32, 72, 220),

            fg_primary: Color::rgb(240, 240, 255),
            fg_secondary: Color::rgb(180, 180, 210),
            fg_muted: Color::rgb(100, 100, 140),
            fg_inverse: Color::rgb(8, 8, 24),

            accent: Color::rgb(0, 210, 255),        // Electric cyan
            accent_hover: Color::rgb(50, 230, 255),
            accent_subtle: Color::rgba(0, 210, 255, 30),

            success: Color::rgb(0, 220, 130),
            warning: Color::rgb(255, 180, 0),
            error: Color::rgb(255, 70, 90),
            info: Color::rgb(100, 140, 255),

            glow: Color::rgba(0, 210, 255, 40),
            glass_tint: Color::rgba(255, 255, 255, 8),
        },
        typography: Typography {
            font_family: "Inter",
            font_size_xs: 11.0,
            font_size_sm: 13.0,
            font_size_md: 15.0,
            font_size_lg: 18.0,
            font_size_xl: 24.0,
            font_size_display: 48.0,
            line_height: 1.5,
            letter_spacing: 0.0,
        },
        spacing: Spacing { xs: 4.0, sm: 8.0, md: 16.0, lg: 24.0, xl: 48.0 },
        radii: Radii { sm: 6.0, md: 12.0, lg: 16.0, xl: 24.0, full: 9999.0 },
        shadows: Shadows {
            sm: Shadow { x: 0.0, y: 2.0, blur: 8.0, spread: 0.0, color: Color::rgba(0, 0, 0, 60) },
            md: Shadow { x: 0.0, y: 4.0, blur: 16.0, spread: 0.0, color: Color::rgba(0, 0, 0, 80) },
            lg: Shadow { x: 0.0, y: 8.0, blur: 32.0, spread: 0.0, color: Color::rgba(0, 0, 0, 100) },
            glow: Shadow { x: 0.0, y: 0.0, blur: 20.0, spread: 4.0, color: Color::rgba(0, 210, 255, 30) },
        },
        blur: BlurStyle { surface: 20.0, elevated: 28.0, overlay: 40.0 },
        animation: AnimationStyle { duration_fast: 150, duration_normal: 300, duration_slow: 500 },
    }
}

/// Aurora — Light theme
/// Ethereal with warm gradients
pub fn aurora() -> Theme {
    Theme {
        name: "Aurora",
        mode: ThemeMode::Light,
        colors: ColorPalette {
            bg_primary: Color::rgb(248, 248, 255),
            bg_secondary: Color::rgb(240, 240, 250),
            bg_tertiary: Color::rgb(230, 232, 245),
            bg_surface: Color::rgba(255, 255, 255, 220),
            bg_elevated: Color::rgba(255, 255, 255, 240),

            fg_primary: Color::rgb(20, 20, 40),
            fg_secondary: Color::rgb(60, 60, 90),
            fg_muted: Color::rgb(140, 140, 170),
            fg_inverse: Color::rgb(248, 248, 255),

            accent: Color::rgb(100, 60, 255),        // Violet
            accent_hover: Color::rgb(120, 80, 255),
            accent_subtle: Color::rgba(100, 60, 255, 20),

            success: Color::rgb(0, 180, 100),
            warning: Color::rgb(220, 150, 0),
            error: Color::rgb(220, 50, 70),
            info: Color::rgb(60, 100, 220),

            glow: Color::rgba(100, 60, 255, 25),
            glass_tint: Color::rgba(255, 255, 255, 180),
        },
        typography: Typography {
            font_family: "Inter",
            font_size_xs: 11.0, font_size_sm: 13.0, font_size_md: 15.0,
            font_size_lg: 18.0, font_size_xl: 24.0, font_size_display: 48.0,
            line_height: 1.5, letter_spacing: 0.0,
        },
        spacing: Spacing { xs: 4.0, sm: 8.0, md: 16.0, lg: 24.0, xl: 48.0 },
        radii: Radii { sm: 6.0, md: 12.0, lg: 16.0, xl: 24.0, full: 9999.0 },
        shadows: Shadows {
            sm: Shadow { x: 0.0, y: 1.0, blur: 4.0, spread: 0.0, color: Color::rgba(0, 0, 0, 15) },
            md: Shadow { x: 0.0, y: 2.0, blur: 8.0, spread: 0.0, color: Color::rgba(0, 0, 0, 20) },
            lg: Shadow { x: 0.0, y: 4.0, blur: 16.0, spread: 0.0, color: Color::rgba(0, 0, 0, 25) },
            glow: Shadow { x: 0.0, y: 0.0, blur: 16.0, spread: 2.0, color: Color::rgba(100, 60, 255, 20) },
        },
        blur: BlurStyle { surface: 16.0, elevated: 24.0, overlay: 32.0 },
        animation: AnimationStyle { duration_fast: 150, duration_normal: 300, duration_slow: 500 },
    }
}

/// Obsidian — OLED dark theme
/// Pure black for power efficiency and contrast
pub fn obsidian() -> Theme {
    let mut theme = nebula();
    theme.name = "Obsidian";
    theme.mode = ThemeMode::Oled;
    theme.colors.bg_primary = Color::rgb(0, 0, 0);
    theme.colors.bg_secondary = Color::rgb(6, 6, 6);
    theme.colors.bg_tertiary = Color::rgb(12, 12, 12);
    theme.colors.bg_surface = Color::rgba(16, 16, 16, 240);
    theme.colors.bg_elevated = Color::rgba(24, 24, 24, 250);
    theme.colors.accent = Color::rgb(0, 255, 200);  // Mint green for OLED pop
    theme
}
