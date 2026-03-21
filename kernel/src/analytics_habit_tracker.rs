extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct HabitTracker {
    habits: Vec<String>,
    streaks: Vec<u32>,
}

impl HabitTracker {
    pub fn new() -> Self {
        HabitTracker {
            habits: Vec::new(),
            streaks: Vec::new(),
        }
    }

    pub fn add_habit(&mut self, habit: String) {
        if !self.habits.contains(&habit) {
            self.habits.push(habit);
            self.streaks.push(0);
        }
    }

    pub fn remove_habit(&mut self, habit: &str) {
        if let Some(index) = self.habits.iter().position(|h| h == habit) {
            self.habits.remove(index);
            self.streaks.remove(index);
        }
    }

    pub fn track_habit(&mut self, habit: &str) {
        if let Some(index) = self.habits.iter().position(|h| h == habit) {
            self.streaks[index] += 1;
        }
    }

    pub fn reset_streak(&mut self, habit: &str) {
        if let Some(index) = self.habits.iter().position(|h| h == habit) {
            self.streaks[index] = 0;
        }
    }

    pub fn get_habit_streak(&self, habit: &str) -> Option<u32> {
        self.habits.iter().position(|h| h == habit).map(|index| self.streaks[index])
    }
}
