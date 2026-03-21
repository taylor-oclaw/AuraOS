extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_client_vendor_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rel_client_vendor_exit() {
    // Cleanup logic for the module
}

pub struct VendorClient {
    name: String,
    products: Vec<String>,
    customers: Vec<String>,
    revenue: u64,
    employees: u32,
}

impl VendorClient {
    pub fn new(name: &str) -> Self {
        VendorClient {
            name: String::from(name),
            products: Vec::new(),
            customers: Vec::new(),
            revenue: 0,
            employees: 0,
        }
    }

    pub fn add_product(&mut self, product_name: &str) {
        self.products.push(String::from(product_name));
    }

    pub fn remove_product(&mut self, product_name: &str) {
        if let Some(index) = self.products.iter().position(|p| p == product_name) {
            self.products.remove(index);
        }
    }

    pub fn add_customer(&mut self, customer_name: &str) {
        self.customers.push(String::from(customer_name));
    }

    pub fn remove_customer(&mut self, customer_name: &str) {
        if let Some(index) = self.customers.iter().position(|c| c == customer_name) {
            self.customers.remove(index);
        }
    }

    pub fn update_revenue(&mut self, amount: u64) {
        self.revenue += amount;
    }

    pub fn get_info(&self) -> String {
        let mut info = format!("Vendor Name: {}\n", self.name);
        info.push_str(&format!("Products: {:?}\n", self.products));
        info.push_str(&format!("Customers: {:?}\n", self.customers));
        info.push_str(&format!("Revenue: ${}\n", self.revenue));
        info.push_str(&format!("Employees: {}\n", self.employees));
        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vendor_client() {
        let mut vendor = VendorClient::new("TechCorp");
        assert_eq!(vendor.name, "TechCorp");
        assert!(vendor.products.is_empty());
        assert!(vendor.customers.is_empty());
        assert_eq!(vendor.revenue, 0);
        assert_eq!(vendor.employees, 0);

        vendor.add_product("Widget A");
        vendor.add_customer("Customer X");
        vendor.update_revenue(10000);

        assert!(!vendor.products.is_empty());
        assert!(!vendor.customers.is_empty());
        assert_eq!(vendor.revenue, 10000);

        vendor.remove_product("Widget A");
        vendor.remove_customer("Customer X");

        assert!(vendor.products.is_empty());
        assert!(vendor.customers.is_empty());

        let info = vendor.get_info();
        assert!(info.contains("Vendor Name: TechCorp"));
        assert!(info.contains("Products: []"));
        assert!(info.contains("Customers: []"));
        assert!(info.contains("Revenue: $10000"));
        assert!(info.contains("Employees: 0"));
    }
}
