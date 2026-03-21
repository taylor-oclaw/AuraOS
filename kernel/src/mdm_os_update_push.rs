extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct MdmOsUpdatePush {
    updates: Vec<String>,
    status: String,
}

impl MdmOsUpdatePush {
    pub fn new() -> Self {
        MdmOsUpdatePush {
            updates: Vec::new(),
            status: String::from("Idle"),
        }
    }

    pub fn add_update(&mut self, update: &str) {
        self.updates.push(update.to_string());
    }

    pub fn get_updates(&self) -> &[String] {
        &self.updates
    }

    pub fn mark_as_downloading(&mut self) {
        self.status = String::from("Downloading");
    }

    pub fn mark_as_installing(&mut self) {
        self.status = String::from("Installing");
    }

    pub fn mark_as_completed(&mut self) {
        self.status = String::from("Completed");
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mdm_os_update_push() {
        let mut update_manager = MdmOsUpdatePush::new();
        assert_eq!(update_manager.get_updates().len(), 0);
        assert_eq!(update_manager.get_status(), "Idle");

        update_manager.add_update("Update 1");
        update_manager.add_update("Update 2");
        assert_eq!(update_manager.get_updates().len(), 2);

        update_manager.mark_as_downloading();
        assert_eq!(update_manager.get_status(), "Downloading");

        update_manager.mark_as_installing();
        assert_eq!(update_manager.get_status(), "Installing");

        update_manager.mark_as_completed();
        assert_eq!(update_manager.get_status(), "Completed");
    }
}
