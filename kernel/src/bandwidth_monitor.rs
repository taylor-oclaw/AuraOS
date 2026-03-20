extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod bandwidth_monitor {
    use super::*;

    pub struct BandwidthMonitor {
        interface_name: String,
        bytes_sent: u64,
        bytes_received: u64,
        packets_sent: u64,
        packets_received: u64,
    }

    impl BandwidthMonitor {
        pub fn new(interface_name: &str) -> Self {
            BandwidthMonitor {
                interface_name: String::from(interface_name),
                bytes_sent: 0,
                bytes_received: 0,
                packets_sent: 0,
                packets_received: 0,
            }
        }

        pub fn update_stats(&mut self, sent_bytes: u64, received_bytes: u64, sent_packets: u64, received_packets: u64) {
            self.bytes_sent += sent_bytes;
            self.bytes_received += received_bytes;
            self.packets_sent += sent_packets;
            self.packets_received += received_packets;
        }

        pub fn get_interface_name(&self) -> &str {
            &self.interface_name
        }

        pub fn get_total_bytes_sent(&self) -> u64 {
            self.bytes_sent
        }

        pub fn get_total_bytes_received(&self) -> u64 {
            self.bytes_received
        }

        pub fn get_total_packets_sent(&self) -> u64 {
            self.packets_sent
        }

        pub fn get_total_packets_received(&self) -> u64 {
            self.packets_received
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bandwidth_monitor::*;

    #[test]
    fn test_bandwidth_monitor() {
        let mut monitor = BandwidthMonitor::new("eth0");
        assert_eq!(monitor.get_interface_name(), "eth0");
        assert_eq!(monitor.get_total_bytes_sent(), 0);
        assert_eq!(monitor.get_total_bytes_received(), 0);
        assert_eq!(monitor.get_total_packets_sent(), 0);
        assert_eq!(monitor.get_total_packets_received(), 0);

        monitor.update_stats(1024, 2048, 5, 10);
        assert_eq!(monitor.get_total_bytes_sent(), 1024);
        assert_eq!(monitor.get_total_bytes_received(), 2048);
        assert_eq!(monitor.get_total_packets_sent(), 5);
        assert_eq!(monitor.get_total_packets_received(), 10);

        monitor.update_stats(512, 1024, 3, 6);
        assert_eq!(monitor.get_total_bytes_sent(), 1536);
        assert_eq!(monitor.get_total_bytes_received(), 3072);
        assert_eq!(monitor.get_total_packets_sent(), 8);
        assert_eq!(monitor.get_total_packets_received(), 16);
    }
}
