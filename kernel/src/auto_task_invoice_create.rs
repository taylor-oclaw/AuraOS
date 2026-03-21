extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let invoice = AutoTaskInvoiceCreate::new(String::from("AI-12345"), 100.0, vec![String::from("Item1"), String::from("Item2")]);
    invoice.add_item(String::from("Item3"));
    invoice.remove_item(String::from("Item2"));
    let total = invoice.calculate_total();
    let description = invoice.get_description();
    // Simulate sending the invoice to a service
    send_invoice_to_service(description, total);
}

struct AutoTaskInvoiceCreate {
    invoice_id: String,
    items: Vec<String>,
    total_amount: f64,
}

impl AutoTaskInvoiceCreate {
    pub fn new(invoice_id: String, initial_total: f64, initial_items: Vec<String>) -> Self {
        AutoTaskInvoiceCreate {
            invoice_id,
            items: initial_items,
            total_amount: initial_total,
        }
    }

    pub fn add_item(&mut self, item_name: String) {
        self.items.push(item_name);
    }

    pub fn remove_item(&mut self, item_name: String) {
        if let Some(index) = self.items.iter().position(|x| *x == item_name) {
            self.items.remove(index);
        }
    }

    pub fn calculate_total(&self) -> f64 {
        // Assuming each item costs $10 for simplicity
        self.items.len() as f64 * 10.0
    }

    pub fn get_description(&self) -> String {
        format!("Invoice ID: {}, Items: {:?}", self.invoice_id, self.items)
    }
}

fn send_invoice_to_service(description: String, total: f64) {
    // Simulate sending the invoice to a service
    println!("Sending invoice: {} with total ${}", description, total);
}
