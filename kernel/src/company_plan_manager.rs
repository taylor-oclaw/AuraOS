extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct CompanyPlanManager {
    plans: Vec<String>,
}

impl CompanyPlanManager {
    pub fn new() -> Self {
        CompanyPlanManager {
            plans: Vec::new(),
        }
    }

    pub fn add_plan(&mut self, plan: String) {
        self.plans.push(plan);
    }

    pub fn remove_plan(&mut self, index: usize) -> Option<String> {
        if index < self.plans.len() {
            Some(self.plans.remove(index))
        } else {
            None
        }
    }

    pub fn get_plan(&self, index: usize) -> Option<&String> {
        self.plans.get(index)
    }

    pub fn list_plans(&self) -> &[String] {
        &self.plans
    }

    pub fn clear_plans(&mut self) {
        self.plans.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_plan_manager() {
        let mut manager = CompanyPlanManager::new();

        assert_eq!(manager.list_plans().len(), 0);

        manager.add_plan(String::from("Plan A"));
        manager.add_plan(String::from("Plan B"));

        assert_eq!(manager.list_plans().len(), 2);
        assert_eq!(manager.get_plan(0), Some(&String::from("Plan A")));
        assert_eq!(manager.get_plan(1), Some(&String::from("Plan B")));

        let removed_plan = manager.remove_plan(0);
        assert_eq!(removed_plan, Some(String::from("Plan A")));
        assert_eq!(manager.list_plans().len(), 1);

        manager.clear_plans();
        assert_eq!(manager.list_plans().len(), 0);
    }
}
