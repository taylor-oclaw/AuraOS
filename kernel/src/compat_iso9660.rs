extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CompatISO9660 {
    volume_descriptor: Vec<u8>,
    root_directory: Vec<u8>,
}

impl CompatISO9660 {
    pub fn new(volume_descriptor: Vec<u8>, root_directory: Vec<u8>) -> Self {
        CompatISO9660 {
            volume_descriptor,
            root_directory,
        }
    }

    pub fn get_volume_label(&self) -> Option<String> {
        if self.volume_descriptor.len() < 34 {
            return None;
        }
        let label_bytes = &self.volume_descriptor[1..=32];
        let mut label = String::new();
        for &byte in label_bytes {
            if byte != 0x20 {
                label.push(byte as char);
            } else {
                break;
            }
        }
        Some(label)
    }

    pub fn get_system_identifier(&self) -> Option<String> {
        if self.volume_descriptor.len() < 41 {
            return None;
        }
        let identifier_bytes = &self.volume_descriptor[33..=40];
        let mut identifier = String::new();
        for &byte in identifier_bytes {
            if byte != 0x20 {
                identifier.push(byte as char);
            } else {
                break;
            }
        }
        Some(identifier)
    }

    pub fn list_root_directory(&self) -> Vec<String> {
        let mut entries = Vec::new();
        let mut offset = 0;
        while offset < self.root_directory.len() {
            if self.root_directory[offset] == 0x00 {
                break;
            }
            if self.root_directory[offset] & 0x01 != 0x01 {
                let entry_length = self.root_directory[offset + 32] as usize;
                let file_name_bytes = &self.root_directory[offset + 8..offset + 8 + entry_length];
                let mut file_name = String::new();
                for &byte in file_name_bytes {
                    if byte == 0x00 || byte == 0x3b {
                        break;
                    }
                    file_name.push(byte as char);
                }
                entries.push(file_name);
            }
            offset += self.root_directory[offset + 32] as usize;
        }
        entries
    }

    pub fn get_file_size(&self, file_name: &str) -> Option<u32> {
        let mut offset = 0;
        while offset < self.root_directory.len() {
            if self.root_directory[offset] == 0x00 {
                break;
            }
            if self.root_directory[offset] & 0x01 != 0x01 {
                let entry_length = self.root_directory[offset + 32] as usize;
                let file_name_bytes = &self.root_directory[offset + 8..offset + 8 + entry_length];
                let mut current_file_name = String::new();
                for &byte in file_name_bytes {
                    if byte == 0x00 || byte == 0x3b {
                        break;
                    }
                    current_file_name.push(byte as char);
                }
                if current_file_name == file_name {
                    let size_high = u32::from_be_bytes([
                        self.root_directory[offset + 40],
                        self.root_directory[offset + 41],
                        self.root_directory[offset + 42],
                        self.root_directory[offset + 43],
                    ];
                    let size_low = u32::from_be_bytes([
                        self.root_directory[offset + 44],
                        self.root_directory[offset + 45],
                        self.root_directory[offset + 46],
                        self.root_directory[offset + 47],
                    ];
                    return Some((size_high << 16) | size_low);
                }
            }
            offset += self.root_directory[offset + 32] as usize;
        }
        None
    }

    pub fn get_file_data(&self, file_name: &str) -> Option<Vec<u8>> {
        // This is a simplified example. In a real implementation, you would need to parse the directory entries,
        // locate the file's extent descriptors, and read the data from the appropriate sectors.
        None
    }
))}
