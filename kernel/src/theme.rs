extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct Theme {
    pub name: String,
    pub bg_color: u32,
    pub fg_color: u32,
    pub accent_color: u32,
    pub window_bg: u32,
    pub window_border: u32,
    pub titlebar_bg: u32,
    pub titlebar_fg: u32,
    pub taskbar_bg: u32,
    pub taskbar_fg: u32,
    pub font_size: u8,
    pub border_width: u8,
    pub corner_radius: u8,
}

pub struct ThemeManager {
    themes: Vec<Theme>,
    active_index: usize,
}

impl ThemeManager {
    pub fn new() -> Self {
        let dark_theme = Theme {
            name: String::from("Dark"),
            bg_color: 0x000080, // navy
            fg_color: 0xFFFFFF, // white
            accent_color: 0x00FFFF, // cyan
            window_bg: 0x000080,
            window_border: 0x000080,
            titlebar_bg: 0x000080,
            titlebar_fg: 0xFFFFFF,
            taskbar_bg: 0x000080,
            taskbar_fg: 0xFFFFFF,
            font_size: 14,
            border_width: 2,
            corner_radius: 5,
        };

        let light_theme = Theme {
            name: String::from("Light"),
            bg_color: 0xFFFFFF, // white
            fg_color: 0x000000, // black
            accent_color: 0x0000FF, // blue
            window_bg: 0xFFFFFF,
            window_border: 0x000000,
            titlebar_bg: 0x0000FF,
            titlebar_fg: 0xFFFFFF,
            taskbar_bg: 0x0000FF,
            taskbar_fg: 0xFFFFFF,
            font_size: 14,
            border_width: 2,
            corner_radius: 5,
        };

        let retro_theme = Theme {
            name: String::from("Retro"),
            bg_color: 0x000000, // black
            fg_color: 0x00FF00, // green
            accent_color: 0x00FF00,
            window_bg: 0x000000,
            window_border: 0x00FF00,
            titlebar_bg: 0x00FF00,
            titlebar_fg: 0x000000,
            taskbar_bg: 0x00FF00,
            taskbar_fg: 0x000000,
            font_size: 14,
            border_width: 2,
            corner_radius: 5,
        };

        ThemeManager {
            themes: vec![dark_theme, light_theme, retro_theme],
            active_index: 0,
        }
    }

    pub fn set_theme(&mut self, name: &str) -> bool {
        if let Some(index) = self.themes.iter().position(|theme| theme.name == name) {
            self.active_index = index;
            true
        } else {
            false
        }
    }

    pub fn active(&self) -> &Theme {
        &self.themes[self.active_index]
    }

    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.push(theme);
    }

    pub fn list_themes(&self) -> Vec<String> {
        self.themes.iter().map(|theme| theme.name.clone()).collect()
    }
}