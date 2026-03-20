extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let mut negotiator = AsfVersionNegotiator::new();
    negotiator.add_version("1.0");
    negotiator.add_version("2.0");
    negotiator.add_version("3.0");

    if negotiator.supports_version("2.0") {
        println!("Version 2.0 is supported.");
    } else {
        println!("Version 2.0 is not supported.");
    }

    let latest = negotiator.get_latest_version();
    println!("Latest version: {}", latest);

    let versions = negotiator.get_all_versions();
    for version in versions.iter() {
        println!("Supported version: {}", version);
    }
}

pub struct AsfVersionNegotiator {
    versions: Vec<String>,
}

impl AsfVersionNegotiator {
    pub fn new() -> Self {
        AsfVersionNegotiator {
            versions: Vec::new(),
        }
    }

    pub fn add_version(&mut self, version: &str) {
        self.versions.push(String::from(version));
    }

    pub fn supports_version(&self, version: &str) -> bool {
        self.versions.contains(&String::from(version))
    }

    pub fn get_latest_version(&self) -> String {
        if let Some(version) = self.versions.iter().max() {
            version.clone()
        } else {
            String::new()
        }
    }

    pub fn get_all_versions(&self) -> Vec<String> {
        self.versions.clone()
    }

    pub fn remove_version(&mut self, version: &str) {
        if let Some(index) = self.versions.iter().position(|v| v == version) {
            self.versions.remove(index);
        }
    }
}
