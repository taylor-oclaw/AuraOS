extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum TilePosition {
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    Maximize,
    LeftThird,
    CenterThird,
    RightThird,
}

pub struct TiledWindow {
    pub surface_id: u64,
    pub position: TilePosition,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub struct TilingManager {
    pub tiled: Vec<TiledWindow>,
    pub screen_w: u32,
    pub screen_h: u32,
    pub gap: u32,
    pub enabled: bool,
}

impl TilingManager {
    pub fn new(sw: u32, sh: u32) -> Self {
        Self {
            tiled: Vec::new(),
            screen_w: sw,
            screen_h: sh,
            gap: 8,
            enabled: true,
        }
    }

    pub fn tile(&mut self, surface_id: u64, pos: TilePosition) {
        let g = self.gap;
        let (x, y, w, h) = match pos {
            TilePosition::Left => (g, g, self.screen_w / 2 - g * 2, self.screen_h - g * 2),
            TilePosition::Right => (self.screen_w / 2 + g, g, self.screen_w / 2 - g * 2, self.screen_h - g * 2),
            TilePosition::TopLeft => (g, g, self.screen_w / 2 - g * 2, self.screen_h / 2 - g * 2),
            TilePosition::TopRight => (self.screen_w / 2 + g, g, self.screen_w / 2 - g * 2, self.screen_h / 2 - g * 2),
            TilePosition::BottomLeft => (g, self.screen_h / 2 + g, self.screen_w / 2 - g * 2, self.screen_h / 2 - g * 2),
            TilePosition::BottomRight => (self.screen_w / 2 + g, self.screen_h / 2 + g, self.screen_w / 2 - g * 2, self.screen_h / 2 - g * 2),
            TilePosition::Center => (self.screen_w / 4, self.screen_h / 4, self.screen_w / 2, self.screen_h / 2),
            TilePosition::Maximize => (0, 0, self.screen_w, self.screen_h),
            TilePosition::LeftThird => (g, g, self.screen_w / 3 - g * 2, self.screen_h - g * 2),
            TilePosition::CenterThird => (self.screen_w / 3 + g, g, self.screen_w / 3 - g * 2, self.screen_h - g * 2),
            TilePosition::RightThird => (self.screen_w * 2 / 3 + g, g, self.screen_w / 3 - g * 2, self.screen_h - g * 2),
        };

        self.tiled.retain(|t| t.surface_id != surface_id);
        self.tiled.push(TiledWindow {
            surface_id,
            position: pos,
            x,
            y,
            w,
            h,
        };
    }

    pub fn untile(&mut self, surface_id: u64) {
        self.tiled.retain(|t| t.surface_id != surface_id);
    }

    pub fn get_tile(&self, surface_id: u64) -> Option<&TiledWindow> {
        self.tiled.iter().find(|t| t.surface_id == surface_id)
    }
)}
