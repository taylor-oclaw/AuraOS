extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Initialize the intrusion detector module
    let mut detector = IntrusionDetector::new();
    
    // Simulate some network traffic
    let packets = vec![
        String::from("normal_traffic"),
        String::from("malicious_activity"),
        String::from("normal_traffic"),
        String::from("suspicious_behavior"),
    ];
    
    for packet in packets {
        detector.process_packet(&packet);
    }
    
    // Analyze the results
    if detector.is_intrusion_detected() {
        println!("Intrusion detected!");
    } else {
        println!("No intrusion detected.");
    }
}

pub struct IntrusionDetector {
    suspicious_packets: Vec<String>,
    threshold: usize,
}

impl IntrusionDetector {
    pub fn new() -> Self {
        IntrusionDetector {
            suspicious_packets: Vec::new(),
            threshold: 2, // Example threshold for detecting an intrusion
        }
    }

    pub fn process_packet(&mut self, packet: &str) {
        if self.is_suspicious(packet) {
            self.suspicious_packets.push(String::from(packet));
        }
    }

    fn is_suspicious(&self, packet: &str) -> bool {
        // Simple heuristic to determine if a packet is suspicious
        packet.contains("malicious") || packet.contains("suspicious")
    }

    pub fn get_suspicious_packets(&self) -> Vec<String> {
        self.suspicious_packets.clone()
    }

    pub fn clear_suspicious_packets(&mut self) {
        self.suspicious_packets.clear();
    }

    pub fn is_intrusion_detected(&self) -> bool {
        self.suspicious_packets.len() >= self.threshold
    }
}