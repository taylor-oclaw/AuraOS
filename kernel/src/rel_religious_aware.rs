extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct ReligiousAwareness {
    beliefs: Vec<String>,
    followers: u32,
    location: String,
}

impl ReligiousAwareness {
    pub fn new(location: &str) -> Self {
        ReligiousAwareness {
            beliefs: Vec::new(),
            followers: 0,
            location: String::from(location),
        }
    }

    pub fn add_belief(&mut self, belief: &str) {
        self.beliefs.push(String::from(belief));
    }

    pub fn get_followers(&self) -> u32 {
        self.followers
    }

    pub fn set_followers(&mut self, followers: u32) {
        self.followers = followers;
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    pub fn list_beliefs(&self) -> Vec<&str> {
        self.beliefs.iter().map(|b| b.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_religious_awareness() {
        let mut ra = ReligiousAwareness::new("Earth");
        assert_eq!(ra.get_followers(), 0);
        assert_eq!(ra.get_location(), "Earth");

        ra.set_followers(1000);
        assert_eq!(ra.get_followers(), 1000);

        ra.add_belief("God is one");
        ra.add_belief("Resurrection");
        let beliefs = ra.list_beliefs();
        assert_eq!(beliefs.len(), 2);
        assert_eq!(beliefs[0], "God is one");
        assert_eq!(beliefs[1], "Resurrection");
    }
}
