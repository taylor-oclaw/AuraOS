extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod calendar_intelligence {
    use super::*;

    pub struct CalendarEvent {
        title: String,
        date: String,
        time: String,
        location: Option<String>,
        description: Option<String>,
    }

    impl CalendarEvent {
        pub fn new(title: &str, date: &str, time: &str) -> Self {
            CalendarEvent {
                title: String::from(title),
                date: String::from(date),
                time: String::from(time),
                location: None,
                description: None,
            }
        }

        pub fn set_location(&mut self, location: &str) {
            self.location = Some(String::from(location));
        }

        pub fn set_description(&mut self, description: &str) {
            self.description = Some(String::from(description));
        }

        pub fn get_title(&self) -> &str {
            &self.title
        }

        pub fn get_date(&self) -> &str {
            &self.date
        }

        pub fn get_time(&self) -> &str {
            &self.time
        }

        pub fn get_location(&self) -> Option<&str> {
            self.location.as_deref()
        }

        pub fn get_description(&self) -> Option<&str> {
            self.description.as_deref()
        }
    }

    pub struct Calendar {
        events: Vec<CalendarEvent>,
    }

    impl Calendar {
        pub fn new() -> Self {
            Calendar { events: Vec::new() }
        }

        pub fn add_event(&mut self, event: CalendarEvent) {
            self.events.push(event);
        }

        pub fn remove_event_by_title(&mut self, title: &str) {
            self.events.retain(|e| e.get_title() != title);
        }

        pub fn get_events_on_date(&self, date: &str) -> Vec<&CalendarEvent> {
            self.events.iter().filter(|e| e.get_date() == date).collect()
        }

        pub fn list_all_events(&self) -> &[CalendarEvent] {
            &self.events
        }
    }
}

#[cfg(test)]
mod tests {
    use super::calendar_intelligence::*;

    #[test]
    fn test_calendar_event_creation() {
        let event = CalendarEvent::new("Meeting", "2023-10-01", "10:00");
        assert_eq!(event.get_title(), "Meeting");
        assert_eq!(event.get_date(), "2023-10-01");
        assert_eq!(event.get_time(), "10:00");
        assert_eq!(event.get_location(), None);
        assert_eq!(event.get_description(), None);
    }

    #[test]
    fn test_calendar_event_modification() {
        let mut event = CalendarEvent::new("Meeting", "2023-10-01", "10:00");
        event.set_location("Conference Room A");
        event.set_description("Discuss project timeline");
        assert_eq!(event.get_location(), Some("Conference Room A"));
        assert_eq!(event.get_description(), Some("Discuss project timeline"));
    }

    #[test]
    fn test_calendar_operations() {
        let mut calendar = Calendar::new();
        let event1 = CalendarEvent::new("Meeting", "2023-10-01", "10:00");
        let event2 = CalendarEvent::new("Lunch", "2023-10-01", "12:00");

        calendar.add_event(event1);
        calendar.add_event(event2);

        assert_eq!(calendar.list_all_events().len(), 2);

        calendar.remove_event_by_title("Meeting");
        assert_eq!(calendar.list_all_events().len(), 1);

        let events_on_date = calendar.get_events_on_date("2023-10-01");
        assert_eq!(events_on_date.len(), 1);
        assert_eq!(events_on_date[0].get_title(), "Lunch");
    }
}
