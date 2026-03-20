extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod skill_rate_limiter {
    use core::time::Duration;
    use alloc::collections::BTreeMap;

    pub struct SkillRateLimiter {
        limits: BTreeMap<String, (u32, Duration)>, // skill_name -> (max_calls, time_window)
        call_records: BTreeMap<(String, u64), u32>,  // (skill_name, timestamp) -> calls_count
    }

    impl SkillRateLimiter {
        pub fn new() -> Self {
            SkillRateLimiter {
                limits: BTreeMap::new(),
                call_records: BTreeMap::new(),
            }
        }

        pub fn add_limit(&mut self, skill_name: String, max_calls: u32, time_window: Duration) {
            self.limits.insert(skill_name, (max_calls, time_window));
        }

        pub fn remove_limit(&mut self, skill_name: &str) {
            self.limits.remove(skill_name);
            self.call_records.retain(|(k, _), _| k.0 != skill_name);
        }

        pub fn can_call(&mut self, skill_name: &str) -> bool {
            if let Some(&(max_calls, time_window)) = self.limits.get(skill_name) {
                let now = Duration::from_secs(core::time::SystemTime::now().duration_since(core::time::UNIX_EPOCH).unwrap().as_secs());
                let start_time = now - time_window;
                let mut total_calls = 0;

                // Remove expired records
                self.call_records.retain(|&(ref k, timestamp), _| {
                    if k.0 == skill_name && timestamp < start_time {
                        false
                    } else {
                        true
                    }
                });

                // Count remaining calls in the time window
                for (_, &count) in self.call_records.iter().filter(|(&(ref k, _), _)| k.0 == skill_name) {
                    total_calls += count;
                }

                if total_calls < max_calls {
                    let now_sec = now.as_secs();
                    *self.call_records.entry((skill_name.to_string(), now_sec)).or_insert(0) += 1;
                    true
                } else {
                    false
                }
            } else {
                true // No limit set, allow call
            }
        }

        pub fn get_limit(&self, skill_name: &str) -> Option<(u32, Duration)> {
            self.limits.get(skill_name).cloned()
        }

        pub fn get_call_count(&self, skill_name: &str) -> u32 {
            let now = Duration::from_secs(core::time::SystemTime::now().duration_since(core::time::UNIX_EPOCH).unwrap().as_secs());
            self.call_records.iter()
                .filter(|(&(ref k, timestamp), _)| k.0 == skill_name && timestamp >= (now - self.limits.get(skill_name).map_or(Duration::from_secs(0), |&(_, tw)| tw)).as_secs())
                .fold(0, |acc, &(_, count)| acc + count)
        }
    }
}
