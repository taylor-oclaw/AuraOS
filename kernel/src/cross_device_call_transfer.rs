extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct CrossDeviceCallTransfer {
    device_id: u32,
    data_buffer: Vec<u8>,
    status: String,
}

impl CrossDeviceCallTransfer {
    pub fn new(device_id: u32) -> Self {
        CrossDeviceCallTransfer {
            device_id,
            data_buffer: Vec::new(),
            status: String::from("Initialized"),
        }
    }

    pub fn set_device_id(&mut self, device_id: u32) {
        self.device_id = device_id;
    }

    pub fn get_device_id(&self) -> u32 {
        self.device_id
    }

    pub fn send_data(&mut self, data: &[u8]) -> bool {
        if !data.is_empty() {
            self.data_buffer.extend_from_slice(data);
            self.status = String::from("Data Sent");
            true
        } else {
            false
        }
    }

    pub fn receive_data(&mut self) -> Vec<u8> {
        let data = self.data_buffer.clone();
        self.data_buffer.clear();
        self.status = String::from("Data Received");
        data
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_device_call_transfer() {
        let mut transfer = CrossDeviceCallTransfer::new(1);
        assert_eq!(transfer.get_device_id(), 1);

        transfer.set_device_id(2);
        assert_eq!(transfer.get_device_id(), 2);

        let data = vec![0x01, 0x02, 0x03];
        assert!(transfer.send_data(&data));
        assert_eq!(transfer.get_status(), "Data Sent");

        let received_data = transfer.receive_data();
        assert_eq!(received_data, data);
        assert_eq!(transfer.get_status(), "Data Received");
    }
}
