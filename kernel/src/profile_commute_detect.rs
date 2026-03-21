extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct CommuteProfile {
    name: String,
    commute_times: Vec<u32>,
    average_time: u32,
    max_time: u32,
    min_time: u32,
}

impl CommuteProfile {
    pub fn new(name: &str) -> Self {
        CommuteProfile {
            name: String::from(name),
            commute_times: Vec::new(),
            average_time: 0,
            max_time: 0,
            min_time: u32::MAX,
        }
    }

    pub fn add_commute_time(&mut self, time: u32) {
        self.commute_times.push(time);
        self.update_statistics();
    }

    fn update_statistics(&mut self) {
        if self.commute_times.is_empty() {
            return;
        }

        let total_time: u32 = self.commute_times.iter().sum();
        self.average_time = total_time / self.commute_times.len() as u32;

        self.max_time = *self.commute_times.iter().max().unwrap();

        self.min_time = *self.commute_times.iter().min().unwrap();
    }

    pub fn get_average_time(&self) -> u32 {
        self.average_time
    }

    pub fn get_max_time(&self) -> u32 {
        self.max_time
    }

    pub fn get_min_time(&self) -> u32 {
        self.min_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commute_profile() {
        let mut profile = CommuteProfile::new("Alice");
        assert_eq!(profile.get_average_time(), 0);
        assert_eq!(profile.get_max_time(), 0);
        assert_eq!(profile.get_min_time(), u32::MAX);

        profile.add_commute_time(30);
        profile.add_commute_time(45);
        profile.add_commute_time(60);

        assert_eq!(profile.get_average_time(), 45);
        assert_eq!(profile.get_max_time(), 60);
        assert_eq!(profile.get_min_time(), 30);
    }
}
