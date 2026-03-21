extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

struct Timer {
    id: u32,
    name: String,
    interval_ms: u64,
    remaining_ms: u64,
    repeating: bool,
    active: bool,
}

struct TimerManager {
    timers: Vec<Timer>,
    next_id: u32,
}

impl TimerManager {
    fn new() -> Self {
        TimerManager {
            timers: Vec::new(),
            next_id: 0,
        }
    }

    fn create_timer(&mut self, name: String, interval_ms: u64, repeating: bool) -> u32 {
        let id = self.next_id;
        self.timers.push(Timer {
            id,
            name,
            interval_ms,
            remaining_ms: interval_ms,
            repeating,
            active: true,
        });
        self.next_id += 1;
        id
    }

    fn cancel_timer(&mut self, id: u32) -> bool {
        if let Some(timer) = self.timers.iter_mut().find(|t| t.id == id) {
            timer.active = false;
            true
        } else {
            false
        }
    }

    fn tick(&mut self, elapsed_ms: u64) -> Vec<u32> {
        let mut fired_ids = Vec::new();
        for timer in self.timers.iter_mut() {
            if timer.active && timer.remaining_ms <= elapsed_ms {
                fired_ids.push(timer.id);
                if timer.repeating {
                    timer.remaining_ms = timer.interval_ms;
                } else {
                    timer.active = false;
                }
            } else if timer.active {
                timer.remaining_ms -= elapsed_ms;
            }
        }
        fired_ids
    }

    fn pause(&mut self, id: u32) -> bool {
        if let Some(timer) = self.timers.iter_mut().find(|t| t.id == id) {
            timer.active = false;
            true
        } else {
            false
        }
    }

    fn resume(&mut self, id: u32) -> bool {
        if let Some(timer) = self.timers.iter_mut().find(|t| t.id == id) {
            timer.active = true;
            true
        } else {
            false
        }
    }

    fn active_count(&self) -> usize {
        self.timers.iter().filter(|t| t.active).count()
    }

    fn list_timers(&self) -> &[Timer] {
        &self.timers
    }
}
