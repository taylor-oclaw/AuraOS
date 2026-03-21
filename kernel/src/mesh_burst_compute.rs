extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn mesh_burst_compute_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_burst_compute_exit() {
    // Cleanup logic for the module
}

pub struct MeshBurstCompute {
    data: Vec<u32>,
    processed_data: Vec<u32>,
}

impl MeshBurstCompute {
    pub fn new(data: Vec<u32>) -> Self {
        MeshBurstCompute {
            data,
            processed_data: Vec::new(),
        }
    }

    pub fn add_data(&mut self, value: u32) {
        self.data.push(value);
    }

    pub fn process_data(&mut self) {
        self.processed_data = self.data.iter().map(|&x| x * 2).collect();
    }

    pub fn get_processed_data(&self) -> &[u32] {
        &self.processed_data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
        self.processed_data.clear();
    }

    pub fn sum_of_processed_data(&self) -> u32 {
        self.processed_data.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_burst_compute() {
        let mut mbc = MeshBurstCompute::new(vec![1, 2, 3]);
        assert_eq!(mbc.get_processed_data(), &[]);
        mbc.process_data();
        assert_eq!(mbc.get_processed_data(), &[2, 4, 6]);
        assert_eq!(mbc.sum_of_processed_data(), 12);
        mbc.add_data(4);
        mbc.process_data();
        assert_eq!(mbc.get_processed_data(), &[2, 4, 6, 8]);
        mbc.clear_data();
        assert_eq!(mbc.get_processed_data(), &[]);
    }
}
