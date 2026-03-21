extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct RuntimeJava {
    class_loader: String,
    heap_size: usize,
    stack_size: usize,
    threads: Vec<String>,
    loaded_classes: Vec<String>,
}

impl RuntimeJava {
    pub fn new(class_loader: &str, heap_size: usize, stack_size: usize) -> Self {
        RuntimeJava {
            class_loader: String::from(class_loader),
            heap_size,
            stack_size,
            threads: Vec::new(),
            loaded_classes: Vec::new(),
        }
    }

    pub fn load_class(&mut self, class_name: &str) -> bool {
        if !self.loaded_classes.contains(&String::from(class_name)) {
            self.loaded_classes.push(String::from(class_name));
            true
        } else {
            false
        }
    }

    pub fn unload_class(&mut self, class_name: &str) -> bool {
        if let Some(index) = self.loaded_classes.iter().position(|x| x == class_name) {
            self.loaded_classes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn create_thread(&mut self, thread_name: &str) -> bool {
        if !self.threads.contains(&String::from(thread_name)) {
            self.threads.push(String::from(thread_name));
            true
        } else {
            false
        }
    }

    pub fn terminate_thread(&mut self, thread_name: &str) -> bool {
        if let Some(index) = self.threads.iter().position(|x| x == thread_name) {
            self.threads.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_loaded_classes(&self) -> Vec<String> {
        self.loaded_classes.clone()
    }
}
