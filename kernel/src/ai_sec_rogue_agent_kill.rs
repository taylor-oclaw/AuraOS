extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("AI Sec Rogue Agent Kill Module Loaded");
    0
}

pub struct RogueAgent {
    name: String,
    id: u32,
    threat_level: u8,
    location: String,
    status: String,
}

impl RogueAgent {
    pub fn new(name: &str, id: u32, threat_level: u8, location: &str) -> Self {
        RogueAgent {
            name: String::from(name),
            id,
            threat_level,
            location: String::from(location),
            status: String::from("Active"),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_threat_level(&self) -> u8 {
        self.threat_level
    }

    pub fn get_location(&self) -> &String {
        &self.location
    }

    pub fn deactivate(&mut self) {
        self.status = String::from("Deactivated");
    }

    pub fn update_location(&mut self, new_location: &str) {
        self.location = String::from(new_location);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rogue_agent_creation() {
        let agent = RogueAgent::new("Agent Smith", 12345, 9, "Sector 7");
        assert_eq!(agent.get_name(), "Agent Smith");
        assert_eq!(agent.get_id(), 12345);
        assert_eq!(agent.get_threat_level(), 9);
        assert_eq!(agent.get_location(), "Sector 7");
        assert_eq!(agent.status, "Active");
    }

    #[test]
    fn test_deactivate_agent() {
        let mut agent = RogueAgent::new("Agent Smith", 12345, 9, "Sector 7");
        agent.deactivate();
        assert_eq!(agent.status, "Deactivated");
    }

    #[test]
    fn test_update_location() {
        let mut agent = RogueAgent::new("Agent Smith", 12345, 9, "Sector 7");
        agent.update_location("Sector 8");
        assert_eq!(agent.get_location(), "Sector 8");
    }
}
