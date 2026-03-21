extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PhotoObjectDetect {
    objects: Vec<String>,
}

impl PhotoObjectDetect {
    pub fn new() -> Self {
        PhotoObjectDetect {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object_name: &str) {
        self.objects.push(String::from(object_name));
    }

    pub fn remove_object(&mut self, object_name: &str) {
        if let Some(index) = self.objects.iter().position(|obj| obj == object_name) {
            self.objects.remove(index);
        }
    }

    pub fn get_objects(&self) -> Vec<String> {
        self.objects.clone()
    }

    pub fn contains_object(&self, object_name: &str) -> bool {
        self.objects.contains(&String::from(object_name))
    }

    pub fn count_objects(&self) -> usize {
        self.objects.len()
    }
}
