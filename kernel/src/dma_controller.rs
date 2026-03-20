extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DmaController {
    channels: Vec<DmaChannel>,
}

impl DmaController {
    pub fn new(num_channels: usize) -> Self {
        let mut channels = Vec::with_capacity(num_channels);
        for _ in 0..num_channels {
            channels.push(DmaChannel::new());
        }
        DmaController { channels }
    }

    pub fn allocate_channel(&mut self) -> Option<usize> {
        if let Some(index) = self.channels.iter().position(|c| !c.is_in_use()) {
            self.channels[index].set_in_use(true);
            Some(index)
        } else {
            None
        }
    }

    pub fn free_channel(&mut self, channel_index: usize) -> Result<(), String> {
        if let Some(channel) = self.channels.get_mut(channel_index) {
            if channel.is_in_use() {
                channel.set_in_use(false);
                Ok(())
            } else {
                Err(String::from("Channel is not in use"))
            }
        } else {
            Err(String::from("Invalid channel index"))
        }
    }

    pub fn transfer_data(&mut self, channel_index: usize, data: &[u8]) -> Result<(), String> {
        if let Some(channel) = self.channels.get_mut(channel_index) {
            if channel.is_in_use() {
                // Simulate DMA transfer
                channel.data = Vec::from(data);
                Ok(())
            } else {
                Err(String::from("Channel is not in use"))
            }
        } else {
            Err(String::from("Invalid channel index"))
        }
    }

    pub fn get_channel_status(&self, channel_index: usize) -> Result<String, String> {
        if let Some(channel) = self.channels.get(channel_index) {
            Ok(String::from("channel status"))
        } else {
            Err(String::from("Invalid channel index"))
        }
    }

    pub fn get_transferred_data(&self, channel_index: usize) -> Result<Vec<u8>, String> {
        if let Some(channel) = self.channels.get(channel_index) {
            Ok(Vec::from(&channel.data[..]))
        } else {
            Err(String::from("Invalid channel index"))
        }
    }
}

struct DmaChannel {
    in_use: bool,
    data: Vec<u8>,
}

impl DmaChannel {
    fn new() -> Self {
        DmaChannel {
            in_use: false,
            data: Vec::new(),
        }
    }

    fn is_in_use(&self) -> bool {
        self.in_use
    }

    fn set_in_use(&mut self, value: bool) {
        self.in_use = value;
    }
}
