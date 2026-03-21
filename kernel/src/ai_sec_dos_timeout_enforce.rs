extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod ai_sec_dos_timeout_enforce {
    use super::*;

    pub struct DosTimeoutEnforcer {
        timeout_list: Vec<(String, u64)>, // (IP address, timestamp)
        max_requests: usize,
        window_size: u64, // in milliseconds
    }

    impl DosTimeoutEnforcer {
        pub fn new(max_requests: usize, window_size: u64) -> Self {
            DosTimeoutEnforcer {
                timeout_list: Vec::new(),
                max_requests,
                window_size,
            }
        }

        pub fn add_request(&mut self, ip_address: &str) {
            let current_time = self.get_current_time();
            self.timeout_list.push((String::from(ip_address), current_time));
            self.cleanup_old_entries(current_time);
        }

        pub fn is_allowed(&self, ip_address: &str) -> bool {
            let current_time = self.get_current_time();
            let count = self
                .timeout_list
                .iter()
                .filter(|&&(ref addr, time)| addr == ip_address && current_time - time <= self.window_size)
                .count();
            count < self.max_requests
        }

        pub fn get_request_count(&self, ip_address: &str) -> usize {
            let current_time = self.get_current_time();
            self.timeout_list
                .iter()
                .filter(|&&(ref addr, time)| addr == ip_address && current_time - time <= self.window_size)
                .count()
        }

        pub fn get_total_requests(&self) -> usize {
            self.timeout_list.len()
        }

        fn cleanup_old_entries(&mut self, current_time: u64) {
            let threshold = current_time - self.window_size;
            self.timeout_list.retain(|&(_, time)| time > threshold);
        }

        fn get_current_time(&self) -> u64 {
            // Placeholder for getting the current time in milliseconds
            // In a real kernel module, you would use a system call or hardware timer
            1000 * (core::arch::x86_64::_rdtsc() / 1_000_000) as u64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dos_timeout_enforcer() {
        let mut enforcer = ai_sec_dos_timeout_enforcer::DosTimeoutEnforcer::new(3, 1000);

        assert!(enforcer.is_allowed("192.168.1.1"));
        enforcer.add_request("192.168.1.1");
        assert!(enforcer.is_allowed("192.168.1.1"));
        enforcer.add_request("192.168.1.1");
        assert!(enforcer.is_allowed("192.168.1.1"));
        enforcer.add_request("192.168.1.1");

        assert!(!enforcer.is_allowed("192.168.1.1"));

        assert_eq!(enforcer.get_request_count("192.168.1.1"), 3);
        assert_eq!(enforcer.get_total_requests(), 3);

        // Simulate time passing
        let current_time = enforcer.get_current_time();
        while enforcer.get_current_time() - current_time < 1000 {
            // Wait for at least 1 second to pass
        }

        assert!(enforcer.is_allowed("192.168.1.1"));
    }
}
