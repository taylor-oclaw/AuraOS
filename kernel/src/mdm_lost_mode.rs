extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod mdm_lost_mode {
    extern crate alloc;
    use alloc::string::String;
    use alloc::vec::Vec;

    pub struct MdmLostMode {
        enabled: bool,
        devices: Vec<String>,
        last_active_time: u64,
        lockout_duration: u64,
        attempts: u32,
    }

    impl MdmLostMode {
        pub fn new() -> Self {
            MdmLostMode {
                enabled: false,
                devices: Vec::new(),
                last_active_time: 0,
                lockout_duration: 3600, // 1 hour in seconds
                attempts: 0,
            }
        }

        pub fn enable(&mut self) {
            self.enabled = true;
            self.attempts = 0;
        }

        pub fn disable(&mut self) {
            self.enabled = false;
        }

        pub fn is_enabled(&self) -> bool {
            self.enabled
        }

        pub fn add_device(&mut self, device_id: &str) {
            if !self.devices.contains(&device_id.to_string()) {
                self.devices.push(device_id.to_string());
            }
        }

        pub fn remove_device(&mut self, device_id: &str) {
            self.devices.retain(|d| d != device_id);
        }

        pub fn get_devices(&self) -> Vec<String> {
            self.devices.clone()
        }

        pub fn record_attempt(&mut self, current_time: u64) {
            if self.enabled {
                self.attempts += 1;
                self.last_active_time = current_time;
            }
        }

        pub fn is_locked_out(&self, current_time: u64) -> bool {
            if !self.enabled || self.attempts == 0 {
                return false;
            }
            current_time - self.last_active_time < self.lockout_duration
        }

        pub fn reset_attempts(&mut self) {
            self.attempts = 0;
        }

        pub fn set_lockout_duration(&mut self, duration: u64) {
            self.lockout_duration = duration;
        }

        pub fn get_lockout_duration(&self) -> u64 {
            self.lockout_duration
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mdm_lost_mode::MdmLostMode;

    #[test]
    fn test_mdm_lost_mode() {
        let mut mdm = MdmLostMode::new();
        assert!(!mdm.is_enabled());

        mdm.enable();
        assert!(mdm.is_enabled());

        mdm.add_device("device1");
        mdm.add_device("device2");
        assert_eq!(mdm.get_devices(), vec![String::from("device1"), String::from("device2")]);

        mdm.remove_device("device1");
        assert_eq!(mdm.get_devices(), vec![String::from("device2")]);

        mdm.record_attempt(100);
        assert_eq!(mdm.is_locked_out(150), true);

        mdm.reset_attempts();
        assert_eq!(mdm.is_locked_out(150), false);

        mdm.set_lockout_duration(600); // 10 minutes
        assert_eq!(mdm.get_lockout_duration(), 600);
    }
}
