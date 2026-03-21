extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_public_info_enrich_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_public_info_enrich_exit() {
    // Cleanup logic for the module
}

pub struct PublicInfoEnricher {
    data: Vec<String>,
}

impl PublicInfoEnricher {
    pub fn new() -> Self {
        PublicInfoEnricher {
            data: Vec::new(),
        }
    }

    pub fn add_info(&mut self, info: String) {
        self.data.push(info);
    }

    pub fn get_info_count(&self) -> usize {
        self.data.len()
    }

    pub fn get_all_info(&self) -> &[String] {
        &self.data
    }

    pub fn remove_info(&mut self, index: usize) -> Option<String> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn clear_all_info(&mut self) {
        self.data.clear();
    }
}

pub extern "C" fn rel_public_info_enrich_add_info(info: *const u8, len: usize) -> i32 {
    let info_str = unsafe { core::slice::from_raw_parts(info, len) };
    if let Ok(s) = String::from_utf8(info_str.to_vec()) {
        let enricher = PublicInfoEnricher::new();
        enricher.add_info(s);
        0
    } else {
        -1
    }
}

pub extern "C" fn rel_public_info_enrich_get_info_count() -> usize {
    let enricher = PublicInfoEnricher::new();
    enricher.get_info_count()
}

pub extern "C" fn rel_public_info_enrich_remove_info(index: usize) -> i32 {
    let mut enricher = PublicInfoEnricher::new();
    if enricher.remove_info(index).is_some() {
        0
    } else {
        -1
    }
}

pub extern "C" fn rel_public_info_enrich_clear_all_info() {
    let mut enricher = PublicInfoEnricher::new();
    enricher.clear_all_info();
}
