extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

pub enum WidgetEvent {
    Click(i32, i32),
    KeyPress(char),
    MouseMove(i32, i32),
}

pub trait Widget {
    fn draw(&self, buf: &mut [u32], stride: usize);
    fn bounds(&self) -> Rect;
    fn handle_event(&mut self, event: &WidgetEvent) -> bool;
}

pub struct Label {
    pub rect: Rect,
    pub text: String,
    pub color: u32,
    pub bg_color: u32,
}

impl Label {
    pub fn new(x: i32, y: i32, text: &str, color: u32) -> Self {
        Self {
            rect: Rect { x, y, w: (text.len() * 8) as u32, h: 16 },
            text: String::from(text),
            color,
            bg_color: 0,
        }
    }
}

impl Widget for Label {
    fn draw(&self, buf: &mut [u32], stride: usize) {
        let x = self.rect.x as usize;
        let y = self.rect.y as usize;
        if self.bg_color != 0 {
            for dy in 0..self.rect.h as usize {
                for dx in 0..self.rect.w as usize {
                    let idx = (y + dy) * stride + (x + dx);
                    if idx < buf.len() { buf[idx] = self.bg_color; }
                }
            }
        }
    }

    fn bounds(&self) -> Rect {
        Rect { x: self.rect.x, y: self.rect.y, w: self.rect.w, h: self.rect.h }
    }

    fn handle_event(&mut self, _event: &WidgetEvent) -> bool { false }
}

pub struct Button {
    pub rect: Rect,
    pub label: String,
    pub bg_color: u32,
    pub hover_color: u32,
    pub pressed: bool,
    pub hovered: bool,
}

impl Button {
    pub fn new(x: i32, y: i32, w: u32, h: u32, label: &str, bg: u32) -> Self {
        Self {
            rect: Rect { x, y, w, h },
            label: String::from(label),
            bg_color: bg,
            hover_color: 0x5E81AC,
            pressed: false,
            hovered: false,
        }
    }
}

impl Widget for Button {
    fn draw(&self, buf: &mut [u32], stride: usize) {
        let color = if self.pressed { 0x88C0D0 } else if self.hovered { self.hover_color } else { self.bg_color };
        let x = self.rect.x as usize;
        let y = self.rect.y as usize;
        for dy in 0..self.rect.h as usize {
            for dx in 0..self.rect.w as usize {
                let idx = (y + dy) * stride + (x + dx);
                if idx < buf.len() { buf[idx] = color; }
            }
        }
    }

    fn bounds(&self) -> Rect {
        Rect { x: self.rect.x, y: self.rect.y, w: self.rect.w, h: self.rect.h }
    }

    fn handle_event(&mut self, event: &WidgetEvent) -> bool {
        match event {
            WidgetEvent::Click(mx, my) => {
                let b = self.bounds();
                if *mx >= b.x && *mx < b.x + b.w as i32 && *my >= b.y && *my < b.y + b.h as i32 {
                    self.pressed = !self.pressed;
                    return true;
                }
                false
            }
            WidgetEvent::MouseMove(mx, my) => {
                let b = self.bounds();
                self.hovered = *mx >= b.x && *mx < b.x + b.w as i32 && *my >= b.y && *my < b.y + b.h as i32;
                false
            }
            _ => false,
        }
    }
}

pub struct TextInput {
    pub rect: Rect,
    pub text: String,
    pub cursor: usize,
    pub focused: bool,
    pub bg_color: u32,
    pub text_color: u32,
}

impl TextInput {
    pub fn new(x: i32, y: i32, w: u32) -> Self {
        Self {
            rect: Rect { x, y, w, h: 20 },
            text: String::new(),
            cursor: 0,
            focused: false,
            bg_color: 0x3B4252,
            text_color: 0xECEFF4,
        }
    }
}

impl Widget for TextInput {
    fn draw(&self, buf: &mut [u32], stride: usize) {
        let x = self.rect.x as usize;
        let y = self.rect.y as usize;
        for dy in 0..self.rect.h as usize {
            for dx in 0..self.rect.w as usize {
                let idx = (y + dy) * stride + (x + dx);
                if idx < buf.len() {
                    buf[idx] = if self.focused { 0x434C5E } else { self.bg_color };
                }
            }
        }
    }

    fn bounds(&self) -> Rect {
        Rect { x: self.rect.x, y: self.rect.y, w: self.rect.w, h: self.rect.h }
    }

    fn handle_event(&mut self, event: &WidgetEvent) -> bool {
        match event {
            WidgetEvent::Click(mx, my) => {
                let b = self.bounds();
                self.focused = *mx >= b.x && *mx < b.x + b.w as i32 && *my >= b.y && *my < b.y + b.h as i32;
                self.focused
            }
            WidgetEvent::KeyPress(ch) => {
                if self.focused {
                    if *ch == '\x08' {
                        if self.cursor > 0 {
                            self.cursor -= 1;
                            self.text.remove(self.cursor);
                        }
                    } else {
                        self.text.insert(self.cursor, *ch);
                        self.cursor += 1;
                    }
                    true
                } else { false }
            }
            _ => false,
        }
    }
}
