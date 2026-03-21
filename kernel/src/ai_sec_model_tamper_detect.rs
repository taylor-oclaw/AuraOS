extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AiSecModelTamperDetect {
    model_name: String,
    version: u32,
    checksum: u64,
    last_modified: u64,
    access_log: Vec<String>,
}

impl AiSecModelTamperDetect {
    pub fn new(model_name: &str, version: u32, checksum: u64, last_modified: u64) -> Self {
        AiSecModelTamperDetect {
            model_name: String::from(model_name),
            version,
            checksum,
            last_modified,
            access_log: Vec::new(),
        }
    }

    pub fn update_checksum(&mut self, new_checksum: u64) {
        self.checksum = new_checksum;
    }

    pub fn log_access(&mut self, user_id: &str) {
        let entry = format!("User {} accessed model {}", user_id, self.model_name);
        self.access_log.push(entry);
    }

    pub fn get_model_info(&self) -> String {
        format!(
            "Model Name: {}, Version: {}, Checksum: {}, Last Modified: {}",
            self.model_name, self.version, self.checksum, self.last_modified
        )
    }

    pub fn verify_integrity(&self, expected_checksum: u64) -> bool {
        self.checksum == expected_checksum
    }

    pub fn get_access_log(&self) -> &Vec<String> {
        &self.access_log
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let model = AiSecModelTamperDetect::new("model1", 1, 0x123456789ABCDEF0, 1633072800);
        assert_eq!(model.model_name, "model1");
        assert_eq!(model.version, 1);
        assert_eq!(model.checksum, 0x123456789ABCDEF0);
        assert_eq!(model.last_modified, 1633072800);
        assert!(model.access_log.is_empty());
    }

    #[test]
    fn test_update_checksum() {
        let mut model = AiSecModelTamperDetect::new("model1", 1, 0x123456789ABCDEF0, 1633072800);
        model.update_checksum(0xFEDCBA9876543210);
        assert_eq!(model.checksum, 0xFEDCBA9876543210);
    }

    #[test]
    fn test_log_access() {
        let mut model = AiSecModelTamperDetect::new("model1", 1, 0x123456789ABCDEF0, 1633072800);
        model.log_access("user1");
        assert_eq!(model.access_log.len(), 1);
        assert_eq!(model.access_log[0], "User user1 accessed model model1");
    }

    #[test]
    fn test_get_model_info() {
        let model = AiSecModelTamperDetect::new("model1", 1, 0x123456789ABCDEF0, 1633072800);
        assert_eq!(
            model.get_model_info(),
            "Model Name: model1, Version: 1, Checksum: 305419896, Last Modified: 1633072800"
        );
    }

    #[test]
    fn test_verify_integrity() {
        let model = AiSecModelTamperDetect::new("model1", 1, 0x123456789ABCDEF0, 1633072800);
        assert!(model.verify_integrity(0x123456789ABCDEF0));
        assert!(!model.verify_integrity(0xFEDCBA9876543210));
    }

    #[test]
    fn test_get_access_log() {
        let mut model = AiSecModelTamperDetect::new("model1", 1, 0x123456789ABCDEF0, 1633072800);
        model.log_access("user1");
        model.log_access("user2");
        let log = model.get_access_log();
        assert_eq!(log.len(), 2);
        assert_eq!(log[0], "User user1 accessed model model1");
        assert_eq!(log[1], "User user2 accessed model model1");
    }
}
