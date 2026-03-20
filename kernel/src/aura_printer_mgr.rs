extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraPrinterMgr {
    printers: Vec<String>,
}

impl AuraPrinterMgr {
    pub fn new() -> Self {
        AuraPrinterMgr {
            printers: Vec::new(),
        }
    }

    pub fn add_printer(&mut self, printer_name: &str) {
        self.printers.push(String::from(printer_name));
    }

    pub fn remove_printer(&mut self, printer_name: &str) {
        if let Some(index) = self.printers.iter().position(|p| p == printer_name) {
            self.printers.remove(index);
        }
    }

    pub fn list_printers(&self) -> Vec<String> {
        self.printers.clone()
    }

    pub fn has_printer(&self, printer_name: &str) -> bool {
        self.printers.contains(&String::from(printer_name))
    }

    pub fn count_printers(&self) -> usize {
        self.printers.len()
    }
}
