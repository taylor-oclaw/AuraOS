extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct ParentalControl {
    user_age: u8,
    unlock_levels: Vec<u8>,
    current_level: usize,
}

impl ParentalControl {
    pub fn new(user_age: u8, unlock_levels: Vec<u8>) -> Self {
        ParentalControl {
            user_age,
            unlock_levels,
            current_level: 0,
        }
    }

    pub fn get_current_level(&self) -> usize {
        self.current_level
    }

    pub fn can_access(&self, level: usize) -> bool {
        if level >= self.unlock_levels.len() {
            return false;
        }
        self.user_age >= self.unlock_levels[level]
    }

    pub fn advance_level(&mut self) -> bool {
        if self.current_level + 1 < self.unlock_levels.len() {
            self.current_level += 1;
            true
        } else {
            false
        }
    }

    pub fn set_user_age(&mut self, age: u8) {
        self.user_age = age;
    }

    pub fn get_unlock_levels(&self) -> &Vec<u8> {
        &self.unlock_levels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let control = ParentalControl::new(10, vec![5, 10, 15]);
        assert_eq!(control.user_age, 10);
        assert_eq!(control.unlock_levels, vec![5, 10, 15]);
        assert_eq!(control.current_level, 0);
    }

    #[test]
    fn test_can_access() {
        let control = ParentalControl::new(10, vec![5, 10, 15]);
        assert!(control.can_access(0));
        assert!(control.can_access(1));
        assert!(!control.can_access(2));
    }

    #[test]
    fn test_advance_level() {
        let mut control = ParentalControl::new(10, vec![5, 10, 15]);
        assert!(control.advance_level());
        assert_eq!(control.current_level, 1);
        assert!(!control.advance_level());
        assert_eq!(control.current_level, 1);
    }

    #[test]
    fn test_set_user_age() {
        let mut control = ParentalControl::new(10, vec![5, 10, 15]);
        control.set_user_age(12);
        assert_eq!(control.user_age, 12);
    }

    #[test]
    fn test_get_unlock_levels() {
        let control = ParentalControl::new(10, vec![5, 10, 15]);
        assert_eq!(*control.get_unlock_levels(), vec![5, 10, 15]);
    }
}
