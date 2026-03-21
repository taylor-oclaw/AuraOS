extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarFocusBlockAuto {
    events: Vec<String>,
    current_focus: Option<usize>,
}

impl CalendarFocusBlockAuto {
    pub fn new() -> Self {
        CalendarFocusBlockAuto {
            events: Vec::new(),
            current_focus: None,
        }
    }

    pub fn add_event(&mut self, event: String) {
        self.events.push(event);
        if self.current_focus.is_none() {
            self.current_focus = Some(0);
        }
    }

    pub fn remove_event(&mut self, index: usize) -> Option<String> {
        if let Some(focus) = self.current_focus {
            if focus >= index {
                self.current_focus = if focus == 0 { None } else { Some(focus - 1) };
            }
        }
        self.events.remove(index)
    }

    pub fn get_current_event(&self) -> Option<&String> {
        self.current_focus.map(|focus| &self.events[focus])
    }

    pub fn move_focus_next(&mut self) {
        if let Some(focus) = self.current_focus {
            if focus < self.events.len() - 1 {
                self.current_focus = Some(focus + 1);
            }
        } else if !self.events.is_empty() {
            self.current_focus = Some(0);
        }
    }

    pub fn move_focus_prev(&mut self) {
        if let Some(focus) = self.current_focus {
            if focus > 0 {
                self.current_focus = Some(focus - 1);
            } else {
                self.current_focus = None;
            }
        }
    }
}
