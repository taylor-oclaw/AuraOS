extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct ThemeColors {
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub error: Color,
    pub surface: Color,
    pub border: Color,
    pub text: Color,
    pub text_dim: Color,
}

pub struct Theme {
    pub name: String,
    pub dark: bool,
    pub colors: ThemeColors,
    pub font_family: String,
    pub font_size: u8,
    pub border_radius: u8,
    pub spacing: u8,
}

pub struct ThemeEngine {
    pub themes: Vec<Theme>,
    pub active_theme: usize,
    pub auto_dark_mode: bool,
}

impl ThemeEngine {
    pub fn new() -> Self {
        let dark = Theme {
            name: String::from("AuraDark"),
            dark: true,
            colors: ThemeColors {
                background: Color { r: 18, g: 18, b: 30, a: 255 },
                foreground: Color { r: 30, g: 30, b: 50, a: 255 },
                primary: Color { r: 0, g: 200, b: 128, a: 255 },
                secondary: Color { r: 100, g: 100, b: 200, a: 255 },
                accent: Color { r: 255, g: 180, b: 0, a: 255 },
                error: Color { r: 255, g: 80, b: 80, a: 255 },
                surface: Color { r: 40, g: 40, b: 60, a: 255 },
                border: Color { r: 60, g: 60, b: 80, a: 255 },
                text: Color { r: 230, g: 230, b: 240, a: 255 },
                text_dim: Color { r: 140, g: 140, b: 160, a: 255 },
            },
            font_family: String::from("Inter"),
            font_size: 14,
            border_radius: 8,
            spacing: 8,
        };

        let light = Theme {
            name: String::from("AuraLight"),
            dark: false,
            colors: ThemeColors {
                background: Color { r: 245, g: 245, b: 250, a: 255 },
                foreground: Color { r: 255, g: 255, b: 255, a: 255 },
                primary: Color { r: 0, g: 150, b: 100, a: 255 },
                secondary: Color { r: 80, g: 80, b: 180, a: 255 },
                accent: Color { r: 200, g: 140, b: 0, a: 255 },
                error: Color { r: 220, g: 50, b: 50, a: 255 },
                surface: Color { r: 235, g: 235, b: 240, a: 255 },
                border: Color { r: 200, g: 200, b: 210, a: 255 },
                text: Color { r: 30, g: 30, b: 40, a: 255 },
                text_dim: Color { r: 120, g: 120, b: 140, a: 255 },
            },
            font_family: String::from("Inter"),
            font_size: 14,
            border_radius: 8,
            spacing: 8,
        };

        Self {
            themes: vec![dark, light],
            active_theme: 0,
            auto_dark_mode: true,
        }
    }

    pub fn current(&self) -> &Theme {
        &self.themes[self.active_theme]
    }

    pub fn switch_theme(&mut self, name: &str) {
        if let Some(idx) = self.themes.iter().position(|t| t.name == name) {
            self.active_theme = idx;
        }
    }

    pub fn is_dark(&self) -> bool {
        self.themes[self.active_theme].dark
    }

    pub fn toggle(&mut self) {
        self.active_theme = (self.active_theme + 1) % self.themes.len();
    }
}
