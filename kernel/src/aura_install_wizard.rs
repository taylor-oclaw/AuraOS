extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_example() {
    // This is a placeholder for an FFI function that could be used to interact with the kernel.
}

struct AuraInstallWizard {
    steps: Vec<String>,
    current_step: usize,
}

impl AuraInstallWizard {
    pub fn new() -> Self {
        let mut wizard = AuraInstallWizard {
            steps: Vec::new(),
            current_step: 0,
        };
        wizard.add_step(String::from("Welcome to the Aura Install Wizard!"));
        wizard.add_step(String::from("Please select your language."));
        wizard.add_step(String::from("Choose your installation type."));
        wizard.add_step(String::from("Configure advanced options."));
        wizard.add_step(String::from("Installation complete. Reboot now?"));
        wizard
    }

    pub fn add_step(&mut self, step: String) {
        self.steps.push(step);
    }

    pub fn get_current_step(&self) -> &String {
        if self.current_step < self.steps.len() {
            &self.steps[self.current_step]
        } else {
            &self.steps[self.steps.len() - 1] // Return the last step if out of bounds
        }
    }

    pub fn next_step(&mut self) {
        if self.current_step < self.steps.len() - 1 {
            self.current_step += 1;
        }
    }

    pub fn previous_step(&mut self) {
        if self.current_step > 0 {
            self.current_step -= 1;
        }
    }

    pub fn is_last_step(&self) -> bool {
        self.current_step == self.steps.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_install_wizard() {
        let mut wizard = AuraInstallWizard::new();
        assert_eq!(wizard.get_current_step(), "Welcome to the Aura Install Wizard!");
        wizard.next_step();
        assert_eq!(wizard.get_current_step(), "Please select your language.");
        wizard.next_step();
        wizard.next_step();
        wizard.previous_step();
        assert_eq!(wizard.get_current_step(), "Choose your installation type.");
        assert!(!wizard.is_last_step());
        wizard.next_step();
        wizard.next_step();
        wizard.next_step();
        assert!(wizard.is_last_step());
    }
}
