extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CalendarNotes {
    notes: Vec<(u32, String)>,
}

impl CalendarNotes {
    pub fn new() -> Self {
        CalendarNotes { notes: Vec::new() }
    }

    pub fn add_note(&mut self, date: u32, note: String) {
        self.notes.push((date, note));
    }

    pub fn get_notes_for_date(&self, date: u32) -> Vec<&String> {
        self.notes.iter()
            .filter(|&&(d, _)| d == date)
            .map(|&(_, ref n)| n)
            .collect()
    }

    pub fn remove_note(&mut self, date: u32, note: &str) -> bool {
        let pos = self.notes.iter().position(|&(d, ref n)| d == date && n == note);
        if let Some(index) = pos {
            self.notes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_all_dates(&self) -> Vec<u32> {
        let mut dates: Vec<u32> = self.notes.iter().map(|&(d, _)| d).collect();
        dates.sort_unstable();
        dates.dedup();
        dates
    }

    pub fn count_notes(&self) -> usize {
        self.notes.len()
    }
}
