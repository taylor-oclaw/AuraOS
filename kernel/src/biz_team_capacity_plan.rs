extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut team = BizTeamCapacityPlan::new();
    team.add_member("Alice");
    team.add_member("Bob");
    team.set_capacity(10);
    team.assign_task("Project X", 5);
    team.assign_task("Project Y", 3);

    loop {}
}

pub struct BizTeamCapacityPlan {
    members: Vec<String>,
    capacity: usize,
    tasks: Vec<(String, usize)>,
}

impl BizTeamCapacityPlan {
    pub fn new() -> Self {
        BizTeamCapacityPlan {
            members: Vec::new(),
            capacity: 0,
            tasks: Vec::new(),
        }
    }

    pub fn add_member(&mut self, name: &str) {
        self.members.push(String::from(name));
    }

    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
    }

    pub fn assign_task(&mut self, task_name: &str, effort: usize) {
        if self.can_assign(effort) {
            self.tasks.push((String::from(task_name), effort));
        }
    }

    pub fn can_assign(&self, effort: usize) -> bool {
        let total_effort = self.tasks.iter().map(|(_, e)| e).sum::<usize>();
        total_effort + effort <= self.capacity
    }

    pub fn get_tasks(&self) -> &Vec<(String, usize)> {
        &self.tasks
    }
}
