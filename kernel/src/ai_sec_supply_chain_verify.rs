extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

struct SupplyChainVerification {
    items: Vec<String>,
    verified: bool,
}

impl SupplyChainVerification {
    pub fn new(items: Vec<String>) -> Self {
        SupplyChainVerification {
            items,
            verified: false,
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item: &str) -> bool {
        if let Some(index) = self.items.iter().position(|x| x == item) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }

    pub fn verify_items(&mut self) {
        // Dummy verification logic
        self.verified = self.items.len() > 0;
    }

    pub fn is_verified(&self) -> bool {
        self.verified
    }

    pub fn list_items(&self) -> Vec<String> {
        self.items.clone()
    }
}

pub extern "C" fn ai_sec_supply_chain_verify_init(items: *const u8, items_len: usize) -> *mut SupplyChainVerification {
    let item_data = unsafe { core::slice::from_raw_parts(items, items_len) };
    let mut supply_chain = SupplyChainVerification::new(Vec::new());

    for i in 0..items_len {
        if let Some(item_end) = item_data[i..].iter().position(|&x| x == 0) {
            let item_str = core::str::from_utf8(&item_data[i..i + item_end]).unwrap_or("");
            supply_chain.add_item(String::from(item_str));
            i += item_end;
        }
    }

    Box::into_raw(Box::new(supply_chain))
}

pub extern "C" fn ai_sec_supply_chain_verify_add_item(module: *mut SupplyChainVerification, item: *const u8) {
    if let Some(supply_chain) = unsafe { module.as_mut() } {
        let c_str = unsafe { core::ffi::CStr::from_ptr(item).to_bytes() };
        if let Ok(item_str) = core::str::from_utf8(c_str) {
            supply_chain.add_item(String::from(item_str));
        }
    }
}

pub extern "C" fn ai_sec_supply_chain_verify_remove_item(module: *mut SupplyChainVerification, item: *const u8) -> bool {
    if let Some(supply_chain) = unsafe { module.as_mut() } {
        let c_str = unsafe { core::ffi::CStr::from_ptr(item).to_bytes() };
        if let Ok(item_str) = core::str::from_utf8(c_str) {
            return supply_chain.remove_item(item_str);
        }
    }
    false
}

pub extern "C" fn ai_sec_supply_chain_verify_verify(module: *mut SupplyChainVerification) {
    if let Some(supply_chain) = unsafe { module.as_mut() } {
        supply_chain.verify_items();
    }
}

pub extern "C" fn ai_sec_supply_chain_verify_is_verified(module: *const SupplyChainVerification) -> bool {
    if let Some(supply_chain) = unsafe { module.as_ref() } {
        return supply_chain.is_verified();
    }
    false
}

pub extern "C" fn ai_sec_supply_chain_verify_list_items(module: *const SupplyChainVerification, items: *mut *const u8, items_len: *mut usize) -> bool {
    if let Some(supply_chain) = unsafe { module.as_ref() } {
        let item_list = supply_chain.list_items();
        let mut item_ptrs: Vec<*const u8> = Vec::new();

        for item in item_list.iter() {
            let c_str = core::ffi::CString::new(item.clone()).unwrap_or_else(|_| core::ffi::CString::new("").unwrap());
            item_ptrs.push(c_str.as_ptr());
        }

        unsafe {
            *items_len = item_ptrs.len();
            *items = item_ptrs.as_ptr();
        }
        return true;
    }
    false
}

pub extern "C" fn ai_sec_supply_chain_verify_destroy(module: *mut SupplyChainVerification) {
    if let Some(supply_chain) = unsafe { module.take() } {
        drop(supply_chain);
    }
}
