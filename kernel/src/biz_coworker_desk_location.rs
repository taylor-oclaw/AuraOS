extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct CoworkerPreferredChannel {
    channels: Vec<String>,
}

impl CoworkerPreferredChannel {
    pub fn new() -> Self {
        CoworkerPreferredChannel {
            channels: Vec::new(),
        }
    }

    pub fn add_channel(&mut self, channel_name: &str) {
        self.channels.push(String::from(channel_name));
    }

    pub fn remove_channel(&mut self, channel_name: &str) {
        if let Some(index) = self.channels.iter().position(|c| c == channel_name) {
            self.channels.remove(index);
        }
    }

    pub fn list_channels(&self) -> Vec<String> {
        self.channels.clone()
    }

    pub fn has_channel(&self, channel_name: &str) -> bool {
        self.channels.contains(&String::from(channel_name))
    }

    pub fn clear_channels(&mut self) {
        self.channels.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coworker_preferred_channel() {
        let mut channel = CoworkerPreferredChannel::new();

        assert_eq!(channel.list_channels(), Vec::<String>::new());

        channel.add_channel("general");
        channel.add_channel("dev");

        assert_eq!(channel.list_channels(), vec!["general".to_string(), "dev".to_string()]);
        assert!(channel.has_channel("general"));
        assert!(!channel.has_channel("random"));

        channel.remove_channel("general");
        assert_eq!(channel.list_channels(), vec!["dev".to_string()]);

        channel.clear_channels();
        assert_eq!(channel.list_channels(), Vec::<String>::new());
    }
}
