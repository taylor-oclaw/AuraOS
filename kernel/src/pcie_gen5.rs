extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn pcie_gen5_init() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn pcie_gen5_exit() -> i32 {
    0
}

pub struct PCIeGen5Device {
    vendor_id: u16,
    device_id: u16,
    class_code: u8,
    subclass_code: u8,
    revision_id: u8,
    capabilities: Vec<String>,
}

impl PCIeGen5Device {
    pub fn new(vendor_id: u16, device_id: u16, class_code: u8, subclass_code: u8, revision_id: u8) -> Self {
        PCIeGen5Device {
            vendor_id,
            device_id,
            class_code,
            subclass_code,
            revision_id,
            capabilities: Vec::new(),
        }
    }

    pub fn add_capability(&mut self, capability: String) {
        self.capabilities.push(capability);
    }

    pub fn get_vendor_id(&self) -> u16 {
        self.vendor_id
    }

    pub fn get_device_id(&self) -> u16 {
        self.device_id
    }

    pub fn get_class_code(&self) -> u8 {
        self.class_code
    }

    pub fn get_subclass_code(&self) -> u8 {
        self.subclass_code
    }

    pub fn get_revision_id(&self) -> u8 {
        self.revision_id
    }

    pub fn list_capabilities(&self) -> Vec<String> {
        self.capabilities.clone()
    }
}
