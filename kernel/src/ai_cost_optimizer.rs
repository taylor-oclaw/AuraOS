extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let optimizer = AICostOptimizer::new();
    optimizer.add_task("task1", 10);
    optimizer.add_task("task2", 5);
    optimizer.optimize_costs();
    let cost = optimizer.get_cost("task1");
    println!("Cost of task1: {}", cost.unwrap());
}

pub struct AICostOptimizer {
    tasks: Vec<(String, u32)>,
}

impl AICostOptimizer {
    pub fn new() -> Self {
        AICostOptimizer { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: &str, cost: u32) {
        self.tasks.push((String::from(name), cost));
    }

    pub fn remove_task(&mut self, name: &str) -> bool {
        let pos = self.tasks.iter().position(|&(ref task_name, _)| task_name == name);
        if let Some(index) = pos {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_cost(&self, name: &str) -> Option<u32> {
        self.tasks.iter().find(|&&(ref task_name, cost)| task_name == name).map(|&(_, cost)| cost)
    }

    pub fn optimize_costs(&mut self) {
        // Simple optimization: double the cost of all tasks
        for (_, cost) in &mut self.tasks {
            *cost *= 2;
        }
    }

    pub fn total_cost(&self) -> u32 {
        self.tasks.iter().map(|&(_, cost)| cost).sum()
    }
}
