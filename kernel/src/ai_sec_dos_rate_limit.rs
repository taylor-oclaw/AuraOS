extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn ai_sec_dos_rate_limit_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn ai_sec_dos_rate_limit_exit() {
    // Cleanup logic for the module
}

pub struct RateLimiter {
    requests: Vec<(u64, u32)>, // (timestamp, count)
    max_requests: u32,
    window_size: u64, // in milliseconds
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_size: u64) -> Self {
        RateLimiter {
            requests: Vec::new(),
            max_requests,
            window_size,
        }
    }

    pub fn allow_request(&mut self, current_time: u64) -> bool {
        self.cleanup_old_requests(current_time);
        if self.requests.len() < self.max_requests as usize {
            self.requests.push((current_time, 1));
            true
        } else {
            false
        }
    }

    pub fn get_request_count(&self, current_time: u64) -> u32 {
        self.cleanup_old_requests(current_time);
        self.requests.iter().map(|&(_, count)| count).sum()
    }

    pub fn set_max_requests(&mut self, max_requests: u32) {
        self.max_requests = max_requests;
    }

    pub fn set_window_size(&mut self, window_size: u64) {
        self.window_size = window_size;
    }

    fn cleanup_old_requests(&mut self, current_time: u64) {
        let cutoff_time = current_time - self.window_size;
        self.requests.retain(|&(timestamp, _)| timestamp > cutoff_time);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(3, 1000);

        // Allow first request
        assert!(limiter.allow_request(100));
        assert_eq!(limiter.get_request_count(100), 1);

        // Allow second request
        assert!(limiter.allow_request(200));
        assert_eq!(limiter.get_request_count(200), 2);

        // Allow third request
        assert!(limiter.allow_request(300));
        assert_eq!(limiter.get_request_count(300), 3);

        // Fourth request should be denied
        assert!(!limiter.allow_request(400));
        assert_eq!(limiter.get_request_count(400), 3);

        // After some time, requests are cleaned up
        assert!(limiter.allow_request(1500));
        assert_eq!(limiter.get_request_count(1500), 1);
    }
}
