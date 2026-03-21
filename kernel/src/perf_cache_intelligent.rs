extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct PerfCacheIntelligent {
    cache_size: usize,
    data_store: Vec<String>,
}

impl PerfCacheIntelligent {
    pub fn new(cache_size: usize) -> Self {
        PerfCacheIntelligent {
            cache_size,
            data_store: Vec::new(),
        }
    }

    pub fn add_data(&mut self, data: String) {
        if self.data_store.len() >= self.cache_size {
            self.data_store.remove(0);
        }
        self.data_store.push(data);
    }

    pub fn get_data(&self, index: usize) -> Option<&String> {
        self.data_store.get(index)
    }

    pub fn remove_data(&mut self, index: usize) -> Option<String> {
        if index < self.data_store.len() {
            Some(self.data_store.remove(index))
        } else {
            None
        }
    }

    pub fn clear_cache(&mut self) {
        self.data_store.clear();
    }

    pub fn cache_size(&self) -> usize {
        self.cache_size
    }
}
