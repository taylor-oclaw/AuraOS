extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn lang_locale_calendar_type_init() {
    // Initialization code if needed
}

pub extern "C" fn lang_locale_calendar_type_exit() {
    // Cleanup code if needed
}

pub struct CalendarType {
    locale: String,
    calendar_name: String,
    days_in_week: Vec<String>,
    months_in_year: Vec<String>,
    start_of_week: u8, // 0 for Sunday, 1 for Monday, etc.
}

impl CalendarType {
    pub fn new(locale: &str, calendar_name: &str) -> Self {
        CalendarType {
            locale: String::from(locale),
            calendar_name: String::from(calendar_name),
            days_in_week: Vec::new(),
            months_in_year: Vec::new(),
            start_of_week: 0,
        }
    }

    pub fn set_days_in_week(&mut self, days: Vec<&str>) {
        self.days_in_week = days.into_iter().map(String::from).collect();
    }

    pub fn set_months_in_year(&mut self, months: Vec<&str>) {
        self.months_in_year = months.into_iter().map(String::from).collect();
    }

    pub fn set_start_of_week(&mut self, start_day: u8) {
        if start_day < 7 {
            self.start_of_week = start_day;
        }
    }

    pub fn get_locale(&self) -> &str {
        &self.locale
    }

    pub fn get_calendar_name(&self) -> &str {
        &self.calendar_name
    }

    pub fn get_days_in_week(&self) -> &[String] {
        &self.days_in_week
    }

    pub fn get_months_in_year(&self) -> &[String] {
        &self.months_in_year
    }

    pub fn get_start_of_week(&self) -> u8 {
        self.start_of_week
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_type() {
        let mut calendar = CalendarType::new("en_US", "Gregorian");
        calendar.set_days_in_week(vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"]);
        calendar.set_months_in_year(vec![
            "January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December"
        ];
        calendar.set_start_of_week(1);

        assert_eq!(calendar.get_locale(), "en_US");
        assert_eq!(calendar.get_calendar_name(), "Gregorian");
        assert_eq!(calendar.get_days_in_week().len(), 7);
        assert_eq!(calendar.get_months_in_year().len(), 12);
        assert_eq!(calendar.get_start_of_week(), 1);
    }
}
