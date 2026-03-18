//! Adaptive layout engine
//! Surfaces are positioned based on context, not manual window management.

use super::Rect;

/// Layout mode determines how surfaces arrange themselves
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    /// Single focused surface fills the screen
    Focus,
    /// Two surfaces side by side
    Split,
    /// Primary surface with floating companions
    Primary,
    /// Grid of equal surfaces
    Grid,
    /// Stacked cards (mobile-like)
    Stack,
    /// Free-form floating (traditional desktop)
    Freeform,
}

/// Calculate layout positions for a set of surfaces
pub fn calculate_layout(
    mode: LayoutMode,
    count: usize,
    screen: Rect,
    gap: f32,
) -> Vec<Rect> {
    match mode {
        LayoutMode::Focus => {
            vec![screen]
        }
        LayoutMode::Split => {
            let half_w = (screen.width - gap) / 2.0;
            vec![
                Rect { x: screen.x, y: screen.y, width: half_w, height: screen.height },
                Rect { x: screen.x + half_w + gap, y: screen.y, width: half_w, height: screen.height },
            ]
        }
        LayoutMode::Grid => {
            let cols = (count as f32).sqrt().ceil() as usize;
            let rows = (count + cols - 1) / cols;
            let cell_w = (screen.width - gap * (cols as f32 - 1.0)) / cols as f32;
            let cell_h = (screen.height - gap * (rows as f32 - 1.0)) / rows as f32;
            
            (0..count).map(|i| {
                let col = i % cols;
                let row = i / cols;
                Rect {
                    x: screen.x + col as f32 * (cell_w + gap),
                    y: screen.y + row as f32 * (cell_h + gap),
                    width: cell_w,
                    height: cell_h,
                }
            }).collect()
        }
        _ => {
            // Default: evenly distributed
            (0..count).map(|i| {
                Rect {
                    x: screen.x + (i as f32 * 50.0),
                    y: screen.y + (i as f32 * 50.0),
                    width: screen.width * 0.6,
                    height: screen.height * 0.6,
                }
            }).collect()
        }
    }
}
