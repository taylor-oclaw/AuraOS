extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_occasion_calendar_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn rel_occasion_calendar_exit() {
    // Cleanup code for the module
}

pub struct OccasionCalendar {
    events: Vec<(String, String)>, // (date, event)
}

impl OccasionCalendar {
    pub fn new() -> Self {
        OccasionCalendar {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, date: &str, event: &str) {
        let date_str = String::from(date);
        let event_str = String::from(event);
        self.events.push((date_str, event_str));
    }

    pub fn remove_event(&mut self, date: &str) -> bool {
        let date_str = String::from(date);
        if let Some(pos) = self.events.iter().position(|&(ref d, _)| *d == date_str) {
            self.events.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_event(&self, date: &str) -> Option<&String> {
        let date_str = String::from(date);
        for (d, e) in &self.events {
            if *d == date_str {
                return Some(e);
            }
        }
        None
    }

    pub fn list_events(&self) -> Vec<(String, String)> {
        self.events.clone()
    }

    pub fn count_events(&self) -> usize {
        self.events.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_occasion_calendar() {
        let mut calendar = OccasionCalendar::new();
        assert_eq!(calendar.count_events(), 0);

        calendar.add_event("2023-10-01", "Birthday");
        assert_eq!(calendar.count_events(), 1);
        assert_eq!(calendar.get_event("2023-10-01").unwrap(), "Birthday");

        calendar.add_event("2023-10-02", "Anniversary");
        assert_eq!(calendar.count_events(), 2);

        let events = calendar.list_events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], (String::from("2023-10-01"), String::from("Birthday")));
        assert_eq!(events[1], (String::from("2023-10-02"), String::from("Anniversary")));

        assert!(calendar.remove_event("2023-10-01"));
        assert_eq!(calendar.count_events(), 1);
        assert_eq!(calendar.get_event("2023-10-01"), None);

        assert!(!calendar.remove_event("2023-10-03"));
    }
}
