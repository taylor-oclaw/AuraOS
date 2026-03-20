extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct IpcSemaphore {
    count: isize,
    waiting_list: Vec<usize>,
}

impl IpcSemaphore {
    pub fn new(initial_count: isize) -> Self {
        IpcSemaphore {
            count: initial_count,
            waiting_list: Vec::new(),
        }
    }

    pub fn wait(&mut self, task_id: usize) {
        if self.count > 0 {
            self.count -= 1;
        } else {
            self.waiting_list.push(task_id);
        }
    }

    pub fn signal(&mut self) -> Option<usize> {
        if let Some(task_id) = self.waiting_list.pop() {
            task_id
        } else {
            self.count += 1;
            None
        }
    }

    pub fn get_count(&self) -> isize {
        self.count
    }

    pub fn is_waiting(&self, task_id: usize) -> bool {
        self.waiting_list.contains(&task_id)
    }

    pub fn clear_waiting_list(&mut self) {
        self.waiting_list.clear();
    }
}
