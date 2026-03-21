extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct SLATracker {
    vendor_name: String,
    sla_metrics: Vec<(String, u64)>,
}

impl SLATracker {
    pub fn new(vendor_name: &str) -> Self {
        SLATracker {
            vendor_name: String::from(vendor_name),
            sla_metrics: Vec::new(),
        }
    }

    pub fn add_metric(&mut self, metric_name: &str, value: u64) {
        self.sla_metrics.push((String::from(metric_name), value));
    }

    pub fn get_vendor_name(&self) -> &str {
        &self.vendor_name
    }

    pub fn get_all_metrics(&self) -> &Vec<(String, u64)> {
        &self.sla_metrics
    }

    pub fn get_metric_value(&self, metric_name: &str) -> Option<u64> {
        self.sla_metrics.iter()
            .find(|&&(ref name, _)| name == metric_name)
            .map(|&(_, value)| value)
    }

    pub fn update_metric(&mut self, metric_name: &str, new_value: u64) -> bool {
        if let Some(index) = self.sla_metrics.iter().position(|&(ref name, _)| name == metric_name) {
            self.sla_metrics[index].1 = new_value;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sla_tracker() {
        let mut tracker = SLATracker::new("VendorA");
        assert_eq!(tracker.get_vendor_name(), "VendorA");

        tracker.add_metric("ResponseTime", 100);
        tracker.add_metric("Uptime", 99.9);

        assert_eq!(tracker.get_all_metrics().len(), 2);
        assert_eq!(tracker.get_metric_value("ResponseTime"), Some(100));
        assert_eq!(tracker.get_metric_value("Uptime"), Some(99));

        assert!(tracker.update_metric("ResponseTime", 50));
        assert_eq!(tracker.get_metric_value("ResponseTime"), Some(50));

        assert!(!tracker.update_metric("Latency", 20));
    }
}
