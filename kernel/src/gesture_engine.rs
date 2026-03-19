extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct TouchPoint {
    pub x: i32,
    pub y: i32,
    pub pressure: f32,
    pub timestamp: u64,
}

pub enum Gesture {
    Tap(TouchPoint),
    DoubleTap(TouchPoint),
    LongPress(TouchPoint),
    Swipe { start: TouchPoint, end: TouchPoint, direction: SwipeDir },
    Pinch { scale: f32 },
    Rotate { angle: f32 },
    ThreeFingerSwipe(SwipeDir),
}

pub enum SwipeDir {
    Up,
    Down,
    Left,
    Right,
}

pub struct GestureAction {
    pub gesture: String,
    pub action: String,
    pub enabled: bool,
}

pub struct GestureEngine {
    pub touch_points: Vec<TouchPoint>,
    pub bindings: Vec<GestureAction>,
    pub gesture_history: Vec<String>,
}

impl GestureEngine {
    pub fn new() -> Self {
        let bindings = vec![
            GestureAction {
                gesture: String::from("three_finger_up"),
                action: String::from("show_all_surfaces"),
                enabled: true,
            },
            GestureAction {
                gesture: String::from("three_finger_down"),
                action: String::from("show_desktop"),
                enabled: true,
            },
            GestureAction {
                gesture: String::from("pinch_in"),
                action: String::from("zoom_out"),
                enabled: true,
            },
            GestureAction {
                gesture: String::from("pinch_out"),
                action: String::from("zoom_in"),
                enabled: true,
            },
            GestureAction {
                gesture: String::from("swipe_left"),
                action: String::from("switch_workspace_left"),
                enabled: true,
            },
            GestureAction {
                gesture: String::from("swipe_right"),
                action: String::from("switch_workspace_right"),
                enabled: true,
            },
        ];

        Self {
            touch_points: Vec::new(),
            bindings,
            gesture_history: Vec::new(),
        }
    }

    pub fn record_touch(&mut self, x: i32, y: i32, pressure: f32, ts: u64) {
        self.touch_points.push(TouchPoint { x, y, pressure, timestamp: ts });
    }

    pub fn recognize(&mut self) -> Option<Gesture> {
        if self.touch_points.len() < 2 {
            return None;
        }

        let fx = self.touch_points[0].x; let fy = self.touch_points[0].y; let fp = self.touch_points[0].pressure; let ft = self.touch_points[0].timestamp;
        let last_idx = self.touch_points.len() - 1; let lx = self.touch_points[last_idx].x; let ly = self.touch_points[last_idx].y; let lp = self.touch_points[last_idx].pressure; let lt = self.touch_points[last_idx].timestamp;

        let dx = lx - fx;
        let dy = ly - fy;

        let dist_sq = (dx * dx + dy * dy) as f32;

        if dist_sq < 100.0 {
            self.touch_points.clear();
            return Some(Gesture::Tap(TouchPoint {
                x: first.x,
                y: first.y,
                pressure: first.pressure,
                timestamp: first.timestamp,
            }));
        }

        let dir = if dx.abs() > dy.abs() {
            if dx > 0 {
                SwipeDir::Right
            } else {
                SwipeDir::Left
            }
        } else {
            if dy > 0 {
                SwipeDir::Down
            } else {
                SwipeDir::Up
            }
        };

        let start = TouchPoint {
            x: first.x,
            y: first.y,
            pressure: first.pressure,
            timestamp: first.timestamp,
        };

        let end = TouchPoint {
            x: last.x,
            y: last.y,
            pressure: last.pressure,
            timestamp: last.timestamp,
        };

        self.touch_points.clear();
        Some(Gesture::Swipe { start, end, direction: dir })
    }

    pub fn get_action(&self, gesture_name: &str) -> Option<&str> {
        self.bindings
            .iter()
            .find(|b| b.gesture == gesture_name && b.enabled)
            .map(|b| b.action.as_str())
    }
}
