extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum UiAction {
    Click(u32, u32),
    Type(String),
    Scroll(i32),
    SwipeUp,
    SwipeDown,
    SwipeLeft,
    SwipeRight,
    LongPress(u32, u32),
    SelectAll,
    Copy,
    Paste,
    PressKey(u8),
    WaitMs(u64)
}

pub struct UiElement {
    pub id: u64,
    pub element_type: String,
    pub text: Option<String>,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub clickable: bool,
    pub visible: bool
}

pub struct GuiAutomation {
    pub action_queue: Vec<UiAction>,
    pub elements: Vec<UiElement>,
    pub recording: bool,
    pub recorded_actions: Vec<UiAction>,
    pub next_id: u64
}

impl GuiAutomation {
    pub fn new() -> Self {
        Self {
            action_queue: Vec::new(),
            elements: Vec::new(),
            recording: false,
            recorded_actions: Vec::new(),
            next_id: 1
        }
    }

    pub fn queue_action(&mut self, action: UiAction) {
        if self.recording {
            self.recorded_actions.push(action.clone());
        }
        self.action_queue.push(action);
    }

    pub fn execute_next(&mut self) -> Option<UiAction> {
        if self.action_queue.is_empty() {
            None
        } else {
            Some(self.action_queue.remove(0))
        }
    }

    pub fn find_element(&self, text: &str) -> Option<&UiElement> {
        self.elements.iter().find(|e| e.text.as_ref().map(|t| t.contains(text)).unwrap_or(false) && e.visible)
    }

    pub fn click_element(&mut self, text: &str) -> bool {
        if let Some(el) = self.elements.iter().find(|e| e.text.as_ref().map(|t| t.contains(text)).unwrap_or(false) && e.clickable) {
            let x = el.x + el.w / 2;
            let y = el.y + el.h / 2;
            self.queue_action(UiAction::Click(x, y));
            true
        } else {
            false
        }
    }

    pub fn start_recording(&mut self) {
        self.recording = true;
        self.recorded_actions.clear();
    }

    pub fn stop_recording(&mut self) -> usize {
        self.recording = false;
        self.recorded_actions.len()
    }

    pub fn pending_actions(&self) -> usize {
        self.action_queue.len()
    }
}
