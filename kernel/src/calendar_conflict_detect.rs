extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct CalendarEvent {
    pub title: String,
    pub start_time: u64, // Unix timestamp in seconds
    pub end_time: u64,   // Unix timestamp in seconds
}

impl CalendarEvent {
    pub fn new(title: &str, start_time: u64, end_time: u64) -> Self {
        CalendarEvent {
            title: String::from(title),
            start_time,
            end_time,
        }
    }

    pub fn overlaps(&self, other: &CalendarEvent) -> bool {
        self.start_time < other.end_time && other.start_time < self.end_time
    }

    pub fn duration(&self) -> u64 {
        self.end_time - self.start_time
    }

    pub fn is_before(&self, other: &CalendarEvent) -> bool {
        self.end_time <= other.start_time
    }

    pub fn is_after(&self, other: &CalendarEvent) -> bool {
        self.start_time >= other.end_time
    }
}

#[repr(C)]
pub struct CalendarConflictDetector {
    events: Vec<CalendarEvent>,
}

impl CalendarConflictDetector {
    pub fn new() -> Self {
        CalendarConflictDetector {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: CalendarEvent) {
        self.events.push(event);
    }

    pub fn has_conflicts(&self) -> bool {
        for i in 0..self.events.len() {
            for j in (i + 1)..self.events.len() {
                if self.events[i].overlaps(&self.events[j]) {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_conflicting_events(&self) -> Vec<(usize, usize)> {
        let mut conflicts = Vec::new();
        for i in 0..self.events.len() {
            for j in (i + 1)..self.events.len() {
                if self.events[i].overlaps(&self.events[j]) {
                    conflicts.push((i, j));
                }
            }
        }
        conflicts
    }

    pub fn remove_event(&mut self, index: usize) -> Option<CalendarEvent> {
        if index < self.events.len() {
            Some(self.events.remove(index))
        } else {
            None
        }
    }
}
