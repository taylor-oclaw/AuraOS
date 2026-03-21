extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut invoice_reminder = InvoiceReminder::new();

    invoice_reminder.add_invoice("Invoice001", 200, "2023-12-01");
    invoice_reminder.add_invoice("Invoice002", 150, "2024-01-15");
    invoice_reminder.add_invoice("Invoice003", 300, "2024-02-20");

    let overdue_invoices = invoice_reminder.get_overdue_invoices();
    for invoice in overdue_invoices {
        // Logic to handle overdue invoices
    }

    loop {}
}

pub struct InvoiceReminder {
    invoices: Vec<Invoice>,
}

impl InvoiceReminder {
    pub fn new() -> Self {
        InvoiceReminder {
            invoices: Vec::new(),
        }
    }

    pub fn add_invoice(&mut self, invoice_id: &str, amount: u32, due_date: &str) {
        let invoice = Invoice {
            id: String::from(invoice_id),
            amount,
            due_date: String::from(due_date),
        };
        self.invoices.push(invoice);
    }

    pub fn get_invoice(&self, invoice_id: &str) -> Option<&Invoice> {
        self.invoices.iter().find(|invoice| invoice.id == invoice_id)
    }

    pub fn remove_invoice(&mut self, invoice_id: &str) {
        self.invoices.retain(|invoice| invoice.id != invoice_id);
    }

    pub fn get_overdue_invoices(&self) -> Vec<&Invoice> {
        let current_date = "2023-11-01"; // Example current date
        self.invoices.iter().filter(|invoice| is_date_before(invoice.due_date.as_str(), current_date)).collect()
    }
}

struct Invoice {
    id: String,
    amount: u32,
    due_date: String,
}

fn is_date_before(date1: &str, date2: &str) -> bool {
    // Simple date comparison logic assuming YYYY-MM-DD format
    let (year1, month1, day1) = parse_date(date1);
    let (year2, month2, day2) = parse_date(date2);

    if year1 < year2 || (year1 == year2 && month1 < month2) || (year1 == year2 && month1 == month2 && day1 < day2) {
        true
    } else {
        false
    }
}

fn parse_date(date: &str) -> (u32, u32, u32) {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() == 3 {
        (
            parts[0].parse().unwrap_or(0),
            parts[1].parse().unwrap_or(0),
            parts[2].parse().unwrap_or(0),
        
    } else {
        (0, 0, 0)
    }
}
