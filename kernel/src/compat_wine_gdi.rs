extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct GdiContext {
    pub brushes: Vec<String>,
    pub pens: Vec<String>,
    pub fonts: Vec<String>,
    pub bitmaps: Vec<String>,
    pub palettes: Vec<String>,
}

impl GdiContext {
    pub fn new() -> Self {
        GdiContext {
            brushes: Vec::new(),
            pens: Vec::new(),
            fonts: Vec::new(),
            bitmaps: Vec::new(),
            palettes: Vec::new(),
        }
    }

    pub fn create_brush(&mut self, name: &str) -> i32 {
        let brush_name = String::from(name);
        self.brushes.push(brush_name);
        0
    }

    pub fn delete_brush(&mut self, index: usize) -> bool {
        if index < self.brushes.len() {
            self.brushes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn create_pen(&mut self, name: &str) -> i32 {
        let pen_name = String::from(name);
        self.pens.push(pen_name);
        0
    }

    pub fn delete_pen(&mut self, index: usize) -> bool {
        if index < self.pens.len() {
            self.pens.remove(index);
            true
        } else {
            false
        }
    }

    pub fn create_font(&mut self, name: &str) -> i32 {
        let font_name = String::from(name);
        self.fonts.push(font_name);
        0
    }

    pub fn delete_font(&mut self, index: usize) -> bool {
        if index < self.fonts.len() {
            self.fonts.remove(index);
            true
        } else {
            false
        }
    }

    pub fn create_bitmap(&mut self, name: &str) -> i32 {
        let bitmap_name = String::from(name);
        self.bitmaps.push(bitmap_name);
        0
    }

    pub fn delete_bitmap(&mut self, index: usize) -> bool {
        if index < self.bitmaps.len() {
            self.bitmaps.remove(index);
            true
        } else {
            false
        }
    }

    pub fn create_palette(&mut self, name: &str) -> i32 {
        let palette_name = String::from(name);
        self.palettes.push(palette_name);
        0
    }

    pub fn delete_palette(&mut self, index: usize) -> bool {
        if index < self.palettes.len() {
            self.palettes.remove(index);
            true
        } else {
            false
        }
    }
}
