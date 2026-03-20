extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

pub struct AuraPermissionPrompt {
    permissions: Vec<String>,
    prompt_message: String,
}

impl AuraPermissionPrompt {
    pub fn new(prompt_message: &str) -> Self {
        AuraPermissionPrompt {
            permissions: Vec::new(),
            prompt_message: String::from(prompt_message),
        }
    }

    pub fn add_permission(&mut self, permission: &str) {
        self.permissions.push(String::from(permission));
    }

    pub fn remove_permission(&mut self, permission: &str) -> bool {
        if let Some(index) = self.permissions.iter().position(|p| p == permission) {
            self.permissions.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&String::from(permission))
    }

    pub fn get_permissions(&self) -> Vec<String> {
        self.permissions.clone()
    }

    pub fn set_prompt_message(&mut self, message: &str) {
        self.prompt_message = String::from(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_permission_prompt() {
        let mut prompt = AuraPermissionPrompt::new("Please grant the following permissions:");

        assert_eq!(prompt.get_permissions().len(), 0);

        prompt.add_permission("read");
        prompt.add_permission("write");

        assert_eq!(prompt.get_permissions().len(), 2);
        assert!(prompt.has_permission("read"));
        assert!(prompt.has_permission("write"));

        assert!(prompt.remove_permission("read"));
        assert!(!prompt.has_permission("read"));
        assert_eq!(prompt.get_permissions().len(), 1);

        prompt.set_prompt_message("New permission request:");
        assert_eq!(prompt.prompt_message, "New permission request:");
    }
}
