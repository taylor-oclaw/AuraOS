extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod a2a_protocol_v2 {
    use super::*;

    pub struct A2AProtocolV2 {
        version: u8,
        data: Vec<u8>,
        status: String,
    }

    impl A2AProtocolV2 {
        pub fn new(version: u8) -> Self {
            A2AProtocolV2 {
                version,
                data: Vec::new(),
                status: String::from("Initialized"),
            }
        }

        pub fn set_data(&mut self, data: Vec<u8>) {
            self.data = data;
            self.status = String::from("Data Set");
        }

        pub fn get_data(&self) -> &Vec<u8> {
            &self.data
        }

        pub fn append_data(&mut self, additional_data: &[u8]) {
            self.data.extend_from_slice(additional_data);
            self.status = String::from("Data Appended");
        }

        pub fn clear_data(&mut self) {
            self.data.clear();
            self.status = String::from("Data Cleared");
        }

        pub fn get_status(&self) -> &str {
            &self.status
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a2a_protocol_v2() {
        let mut protocol = a2a_protocol_v2::A2AProtocolV2::new(2);
        assert_eq!(protocol.get_version(), 2);
        assert_eq!(protocol.get_status(), "Initialized");

        let data = vec![1, 2, 3];
        protocol.set_data(data.clone());
        assert_eq!(protocol.get_data(), &data);
        assert_eq!(protocol.get_status(), "Data Set");

        let additional_data = vec![4, 5];
        protocol.append_data(&additional_data);
        assert_eq!(protocol.get_data(), &[1, 2, 3, 4, 5]);
        assert_eq!(protocol.get_status(), "Data Appended");

        protocol.clear_data();
        assert!(protocol.get_data().is_empty());
        assert_eq!(protocol.get_status(), "Data Cleared");
    }
}
