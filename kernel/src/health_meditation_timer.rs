extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct HealthMeditationTimer {
    duration: u32,
    remaining_time: u32,
    is_active: bool,
}

impl HealthMeditationTimer {
    pub fn new(duration: u32) -> Self {
        HealthMeditationTimer {
            duration,
            remaining_time: duration,
            is_active: false,
        }
    }

    pub fn start(&mut self) {
        if !self.is_active {
            self.remaining_time = self.duration;
            self.is_active = true;
        }
    }

    pub fn stop(&mut self) {
        self.is_active = false;
    }

    pub fn reset(&mut self) {
        self.remaining_time = self.duration;
    }

    pub fn tick(&mut self) -> bool {
        if self.is_active && self.remaining_time > 0 {
            self.remaining_time -= 1;
            return true;
        }
        false
    }

    pub fn get_remaining_time(&self) -> u32 {
        self.remaining_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_creation() {
        let timer = HealthMeditationTimer::new(60);
        assert_eq!(timer.duration, 60);
        assert_eq!(timer.remaining_time, 60);
        assert!(!timer.is_active);
    }

    #[test]
    fn test_timer_start_stop_reset() {
        let mut timer = HealthMeditationTimer::new(30);
        timer.start();
        assert!(timer.is_active);

        timer.stop();
        assert!(!timer.is_active);

        timer.reset();
        assert_eq!(timer.remaining_time, 30);
    }

    #[test]
    fn test_timer_tick() {
        let mut timer = HealthMeditationTimer::new(5);
        timer.start();

        for _ in 0..4 {
            assert!(timer.tick());
        }
        assert_eq!(timer.get_remaining_time(), 1);

        assert!(timer.tick());
        assert!(!timer.is_active);
        assert_eq!(timer.get_remaining_time(), 0);
    }
}
