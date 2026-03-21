extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut sbom = SBOM::new();
    sbom.add_component("AI-Engine", "1.0.0");
    sbom.add_license("MIT");
    sbom.set_supplier("Alibaba Cloud");
    sbom.set_description("AI-native operating system kernel component");

    // Example usage of the SBOM struct
    println!("Component: {}", sbom.get_component(0).unwrap());
    println!("License: {}", sbom.get_license().unwrap());
    println!("Supplier: {}", sbom.get_supplier().unwrap());
    println!("Description: {}", sbom.get_description().unwrap());

    loop {}
}

pub struct SBOM {
    components: Vec<String>,
    license: Option<String>,
    supplier: Option<String>,
    description: Option<String>,
}

impl SBOM {
    pub fn new() -> Self {
        SBOM {
            components: Vec::new(),
            license: None,
            supplier: None,
            description: None,
        }
    }

    pub fn add_component(&mut self, name: &str, version: &str) {
        let component = format!("{}-{}", name, version);
        self.components.push(component);
    }

    pub fn get_component(&self, index: usize) -> Option<&String> {
        self.components.get(index)
    }

    pub fn add_license(&mut self, license: &str) {
        self.license = Some(license.to_string());
    }

    pub fn get_license(&self) -> Option<&String> {
        self.license.as_ref()
    }

    pub fn set_supplier(&mut self, supplier: &str) {
        self.supplier = Some(supplier.to_string());
    }

    pub fn get_supplier(&self) -> Option<&String> {
        self.supplier.as_ref()
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }

    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}
