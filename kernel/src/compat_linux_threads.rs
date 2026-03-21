extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CompatLinuxThreads {
    threads: Vec<Thread>,
}

impl CompatLinuxThreads {
    pub fn new() -> Self {
        CompatLinuxThreads {
            threads: Vec::new(),
        }
    }

    pub fn create_thread(&mut self, name: String) -> usize {
        let thread = Thread::new(name);
        self.threads.push(thread);
        self.threads.len() - 1
    }

    pub fn get_thread_name(&self, index: usize) -> Option<&String> {
        self.threads.get(index).map(|t| &t.name)
    }

    pub fn start_thread(&mut self, index: usize) -> Result<(), &'static str> {
        if let Some(thread) = self.threads.get_mut(index) {
            thread.state = ThreadState::Running;
            Ok(())
        } else {
            Err("Thread not found")
        }
    }

    pub fn stop_thread(&mut self, index: usize) -> Result<(), &'static str> {
        if let Some(thread) = self.threads.get_mut(index) {
            thread.state = ThreadState::Stopped;
            Ok(())
        } else {
            Err("Thread not found")
        }
    }

    pub fn list_threads(&self) -> Vec<&String> {
        self.threads.iter().map(|t| &t.name).collect()
    }
}

struct Thread {
    name: String,
    state: ThreadState,
}

impl Thread {
    fn new(name: String) -> Self {
        Thread {
            name,
            state: ThreadState::Stopped,
        }
    }
}

enum ThreadState {
    Running,
    Stopped,
}
