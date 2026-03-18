//! AuraOS Surface UI Framework
//! 
//! A declarative, GPU-accelerated UI framework for building surfaces.
//! Inspired by SwiftUI and Iced, but designed for an OS, not apps.
//! 
//! Key concepts:
//! - Views: Composable UI primitives (text, image, button, container)
//! - Styles: Theme-aware styling via the theme engine
//! - Layout: Flex-based layout system
//! - State: Reactive state management
//! - Animation: Built-in transition support

pub mod view;
pub mod style;
pub mod flex;
pub mod text;
pub mod widgets;
pub mod state;

/// Size constraint passed during layout
#[derive(Debug, Clone, Copy)]
pub struct Constraints {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl Constraints {
    pub fn unbounded() -> Self {
        Self {
            min_width: 0.0,
            max_width: f32::INFINITY,
            min_height: 0.0,
            max_height: f32::INFINITY,
        }
    }

    pub fn exact(width: f32, height: f32) -> Self {
        Self {
            min_width: width,
            max_width: width,
            min_height: height,
            max_height: height,
        }
    }
}

/// Computed size after layout
#[derive(Debug, Clone, Copy, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

/// Position + Size
#[derive(Debug, Clone, Copy, Default)]
pub struct Frame {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Edge insets (padding/margin)
#[derive(Debug, Clone, Copy, Default)]
pub struct EdgeInsets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl EdgeInsets {
    pub const fn all(v: f32) -> Self {
        Self { top: v, right: v, bottom: v, left: v }
    }
    pub const fn symmetric(h: f32, v: f32) -> Self {
        Self { top: v, right: h, bottom: v, left: h }
    }
}

/// Alignment within a container
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}
