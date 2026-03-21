extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut budget = FamilyHubBudgetShare::new();
    budget.add_member("Alice");
    budget.add_member("Bob");
    budget.set_budget(1000);
    budget.allocate("Alice", 300);
    budget.allocate("Bob", 200);
    budget.print_allocations();

    loop {}
}

pub struct FamilyHubBudgetShare {
    members: Vec<String>,
    budget: u32,
    allocations: Vec<(String, u32)>,
}

impl FamilyHubBudgetShare {
    pub fn new() -> Self {
        FamilyHubBudgetShare {
            members: Vec::new(),
            budget: 0,
            allocations: Vec::new(),
        }
    }

    pub fn add_member(&mut self, name: &str) {
        self.members.push(String::from(name));
    }

    pub fn set_budget(&mut self, amount: u32) {
        self.budget = amount;
    }

    pub fn allocate(&mut self, member: &str, amount: u32) -> bool {
        if self.members.contains(&String::from(member)) && amount <= self.budget {
            self.allocations.push((String::from(member), amount));
            self.budget -= amount;
            true
        } else {
            false
        }
    }

    pub fn get_remaining_budget(&self) -> u32 {
        self.budget
    }

    pub fn print_allocations(&self) {
        for (member, amount) in &self.allocations {
            // Simulate printing to a log or console
            unsafe {
                let message = String::from("info");
                // Assuming there's a kernel function to print strings
                // kernel_print(message.as_ptr(), message.len());
            }
        }
    }
}
