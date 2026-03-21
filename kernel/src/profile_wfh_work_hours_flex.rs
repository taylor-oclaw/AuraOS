extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct WorkHours {
    name: String,
    hours_worked: u32,
    flex_hours: i32,
    target_hours: u32,
    overtime_allowed: bool,
}

impl WorkHours {
    pub fn new(name: &str, target_hours: u32) -> Self {
        WorkHours {
            name: String::from(name),
            hours_worked: 0,
            flex_hours: 0,
            target_hours,
            overtime_allowed: true,
        }
    }

    pub fn record_hours(&mut self, hours: u32) {
        if self.overtime_allowed || hours <= self.target_hours {
            self.hours_worked += hours;
        } else {
            // Handle overtime policy
            self.flex_hours -= (hours - self.target_hours) as i32;
            self.hours_worked = self.target_hours;
        }
    }

    pub fn adjust_target(&mut self, new_target: u32) {
        self.target_hours = new_target;
    }

    pub fn allow_overtime(&mut self, allowed: bool) {
        self.overtime_allowed = allowed;
    }

    pub fn get_summary(&self) -> String {
        let mut summary = String::from("info");
        summary.push_str(&String::from("info"));
        summary.push_str(&String::from("info"));
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_hours() {
        let mut work_hours = WorkHours::new("Alice", 8);
        work_hours.record_hours(6);
        assert_eq!(work_hours.hours_worked, 6);

        work_hours.record_hours(3);
        assert_eq!(work_hours.hours_worked, 8);
        assert_eq!(work_hours.flex_hours, -1);
    }

    #[test]
    fn test_adjust_target() {
        let mut work_hours = WorkHours::new("Bob", 8);
        work_hours.adjust_target(9);
        assert_eq!(work_hours.target_hours, 9);
    }

    #[test]
    fn test_allow_overtime() {
        let mut work_hours = WorkHours::new("Charlie", 8);
        work_hours.allow_overtime(false);
        assert!(!work_hours.overtime_allowed);

        work_hours.record_hours(10);
        assert_eq!(work_hours.hours_worked, 8);
        assert_eq!(work_hours.flex_hours, -2);
    }

    #[test]
    fn test_get_summary() {
        let mut work_hours = WorkHours::new("David", 8);
        work_hours.record_hours(7);
        work_hours.adjust_target(9);
        work_hours.allow_overtime(true);

        let summary = work_hours.get_summary();
        assert!(summary.contains("Name: David"));
        assert!(summary.contains("Target Hours: 9"));
        assert!(summary.contains("Hours Worked: 7"));
        assert!(summary.contains("Flex Hours: 0"));
        assert!(summary.contains("Overtime Allowed: true"));
    }
}
