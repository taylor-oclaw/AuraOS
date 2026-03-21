extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

#[no_std]
mod ai_sec_model_fingerprint {
    use core::fmt::{Debug, Formatter};
    use core::ptr;

    pub struct ModelFingerprint {
        model_name: String,
        fingerprint_data: Vec<u8>,
        version: u32,
        is_valid: bool,
    }

    impl ModelFingerprint {
        pub fn new(model_name: &str, fingerprint_data: &[u8], version: u32) -> Self {
            ModelFingerprint {
                model_name: String::from(model_name),
                fingerprint_data: Vec::from(fingerprint_data),
                version,
                is_valid: false,
            }
        }

        pub fn validate(&mut self) {
            // Simple validation logic for demonstration purposes
            if self.fingerprint_data.len() > 0 && self.version > 0 {
                self.is_valid = true;
            }
        }

        pub fn is_model_valid(&self) -> bool {
            self.is_valid
        }

        pub fn get_fingerprint_data(&self) -> &[u8] {
            &self.fingerprint_data
        }

        pub fn update_fingerprint_data(&mut self, new_data: &[u8]) {
            self.fingerprint_data.clear();
            self.fingerprint_data.extend_from_slice(new_data);
            self.validate(); // Re-validate after updating data
        }
    }

    impl Debug for ModelFingerprint {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ModelFingerprint")
                .field("model_name", &self.model_name)
                .field("fingerprint_data_len", &self.fingerprint_data.len())
                .field("version", &self.version)
                .field("is_valid", &self.is_valid)
                .finish()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_model_fingerprint() {
            let model_name = "TestModel";
            let fingerprint_data = vec![0x1, 0x2, 0x3];
            let version = 1;

            let mut mf = ModelFingerprint::new(model_name, &fingerprint_data, version);
            assert!(!mf.is_model_valid());

            mf.validate();
            assert!(mf.is_model_valid());

            let new_fingerprint_data = vec![0x4, 0x5, 0x6];
            mf.update_fingerprint_data(&new_fingerprint_data);
            assert_eq!(mf.get_fingerprint_data(), &new_fingerprint_data);
        }
    }
}
