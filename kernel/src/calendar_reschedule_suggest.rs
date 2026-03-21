extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarRescheduleSuggest {
    events: Vec<Event>,
}

impl CalendarRescheduleSuggest {
    pub fn new() -> Self {
        CalendarRescheduleSuggest {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn remove_event(&mut self, index: usize) -> Option<Event> {
        if index < self.events.len() {
            Some(self.events.remove(index))
        } else {
            None
        }
    }

    pub fn get_events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn suggest_reschedule(&self, time_slot: TimeSlot) -> Option<&Event> {
        self.events.iter().find(|event| event.time_slot.overlaps(&time_slot))
    }

    pub fn find_conflicts(&self) -> Vec<&Event> {
        let mut conflicts = Vec::new();
        for i in 0..self.events.len() {
            for j in (i + 1)..self.events.len() {
                if self.events[i].time_slot.overlaps(&self.events[j].time_slot) {
                    conflicts.push(&self.events[i]);
                    break;
                }
            }
        }
        conflicts
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    name: String,
    time_slot: TimeSlot,
}

impl Event {
    pub fn new(name: String, start_time: u32, end_time: u32) -> Self {
        Event {
            name,
            time_slot: TimeSlot::new(start_time, end_time),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_time_slot(&self) -> &TimeSlot {
        &self.time_slot
    }
}

#[derive(Debug, Clone)]
pub struct TimeSlot {
    start_time: u32,
    end_time: u32,
}

impl TimeSlot {
    pub fn new(start_time: u32, end_time: u32) -> Self {
        assert!(start_time < end_time, "Start time must be before end time");
        TimeSlot {
            start_time,
            end_time,
        }
    }

    pub fn overlaps(&self, other: &TimeSlot) -> bool {
        self.start_time < other.end_time && other.start_time < self.end_time
    }
}
