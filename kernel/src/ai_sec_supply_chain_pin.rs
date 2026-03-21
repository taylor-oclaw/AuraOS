extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct SupplyChainPin {
    items: Vec<String>,
    suppliers: Vec<String>,
    consumers: Vec<String>,
    transactions: Vec<(String, String, u32)>, // (item, supplier, quantity)
}

impl SupplyChainPin {
    pub fn new() -> Self {
        SupplyChainPin {
            items: Vec::new(),
            suppliers: Vec::new(),
            consumers: Vec::new(),
            transactions: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: String) {
        if !self.items.contains(&item) {
            self.items.push(item);
        }
    }

    pub fn add_supplier(&mut self, supplier: String) {
        if !self.suppliers.contains(&supplier) {
            self.suppliers.push(supplier);
        }
    }

    pub fn add_consumer(&mut self, consumer: String) {
        if !self.consumers.contains(&consumer) {
            self.consumers.push(consumer);
        }
    }

    pub fn record_transaction(&mut self, item: String, supplier: String, quantity: u32) {
        if self.items.contains(&item) && self.suppliers.contains(&supplier) {
            self.transactions.push((item, supplier, quantity));
        }
    }

    pub fn get_transactions(&self) -> Vec<(String, String, u32)> {
        self.transactions.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supply_chain_pin() {
        let mut supply_chain = SupplyChainPin::new();
        supply_chain.add_item(String::from("Widget"));
        supply_chain.add_supplier(String::from("SupplierA"));
        supply_chain.add_consumer(String::from("ConsumerB"));
        supply_chain.record_transaction(String::from("Widget"), String::from("SupplierA"), 100);

        let transactions = supply_chain.get_transactions();
        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0], (String::from("Widget"), String::from("SupplierA"), 100));
    }
}
