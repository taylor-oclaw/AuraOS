extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VymatikDetect {
    vymatik_signature: String,
}

impl VymatikDetect {
    pub fn new() -> Self {
        VymatikDetect {
            vymatik_signature: String::from("VYMATIK"),
        }
    }

    pub fn get_vymatik_signature(&self) -> &str {
        self.vymatik_signature.as_str()
    }

    pub fn is_vymatik_detected(&self, binary_data: &[u8]) -> bool {
        let mut vymatik_found = false;
        for byte in binary_data.iter() {
            if *byte == 0x56 as u8 && !vymatik_found {
                continue;
            }
            if *byte == 0x59 as u8 && !vymatik_found {
                continue;
            }
            if *byte == 0x4d as u8 && !vymatik_found {
                continue;
            }
            if *byte == 0x41 as u8 && !vymatik_found {
                continue;
            }
            if *byte == 0x54 as u8 && !vymatik_found {
                vymatik_found = true;
            } else {
                break;
            }
        }
        vymatik_found
    }

    pub fn get_vymatik_signature_length(&self) -> usize {
        self.vymatik_signature.len()
    }

    pub fn is_binary_data_valid_for_detection(&self, binary_data: &[u8]) -> bool {
        if binary_data.is_empty() || binary_data.len() < 5 {
            return false;
        }
        let mut vymatik_found = false;
        for byte in binary_data.iter().take(5) {
            match *byte {
                0x56 => continue,
                0x59 => continue,
                0x4d => continue,
                0x41 => continue,
                _ => break,
            }
            vymatik_found = true;
        }
        vymatik_found
    }

    pub fn get_detected_vymatik_signature(&self, binary_data: &[u8]) -> Option<&str> {
        if self.is_binary_data_valid_for_detection(binary_data) && self.is_vymatik_detected(binary_data) {
            let mut detected_signature = String::new();
            for byte in binary_data.iter().take(5) {
                match *byte {
                    0x56 => detected_signature.push('V'),
                    0x59 => detected_signature.push('Y'),
                    0x4d => detected_signature.push('M'),
                    0x41 => detected_signature.push('A'),
                    _ => break,
                }
            }
            Some(detected_signature.as_str())
        } else {
            None
        }
    }
}