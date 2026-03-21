extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum DragPayload {
    File(String),
    Text(String),
    Image(Vec<u8>),
    Custom {
        mime_type: String,
        data: Vec<u8>,
    },
}

pub enum DropTarget {
    Surface(u64),
    Desktop,
    Trash,
    ExternalApp(String),
}

pub struct DragState {
    pub active: bool,
    pub payload: Option<DragPayload>,
    pub source_surface: Option<u64>,
    pub start_x: i32,
    pub start_y: i32,
    pub current_x: i32,
    pub current_y: i32,
    pub valid_targets: Vec<DropTarget>,
}

pub struct DragDropManager {
    pub state: DragState,
    pub drop_history: Vec<String>,
}

impl DragDropManager {
    pub fn new() -> Self {
        Self {
            state: DragState {
                active: false,
                payload: None,
                source_surface: None,
                start_x: 0,
                start_y: 0,
                current_x: 0,
                current_y: 0,
                valid_targets: Vec::new(),
            },
            drop_history: Vec::new(),
        }
    }

    pub fn start_drag(&mut self, payload: DragPayload, surface: u64, x: i32, y: i32) {
        self.state.active = true;
        self.state.payload = Some(payload);
        self.state.source_surface = Some(surface);
        self.state.start_x = x;
        self.state.start_y = y;
        self.state.current_x = x;
        self.state.current_y = y;
    }

    pub fn update_position(&mut self, x: i32, y: i32) {
        self.state.current_x = x;
        self.state.current_y = y;
    }

    pub fn drop_on(&mut self, target: &str) {
        self.state.active = false;
        self.state.payload = None;
        self.drop_history.push(String::from(target));
    }

    pub fn cancel(&mut self) {
        self.state.active = false;
        self.state.payload = None;
    }

    pub fn is_dragging(&self) -> bool {
        self.state.active
    }
}
