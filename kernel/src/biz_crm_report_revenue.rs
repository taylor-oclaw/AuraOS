extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct BizCrmReportRevenue {
    customer_id: u32,
    revenue: Vec<u64>,
    report_date: String,
    total_revenue: u64,
    average_revenue: f64,
}

impl BizCrmReportRevenue {
    pub fn new(customer_id: u32, revenue: Vec<u64>, report_date: &str) -> Self {
        let total_revenue = revenue.iter().sum();
        let average_revenue = if !revenue.is_empty() {
            total_revenue as f64 / revenue.len() as f64
        } else {
            0.0
        };
        BizCrmReportRevenue {
            customer_id,
            revenue,
            report_date: String::from(report_date),
            total_revenue,
            average_revenue,
        }
    }

    pub fn add_revenue(&mut self, amount: u64) {
        self.revenue.push(amount);
        self.total_revenue += amount;
        self.average_revenue = self.total_revenue as f64 / self.revenue.len() as f64;
    }

    pub fn get_customer_id(&self) -> u32 {
        self.customer_id
    }

    pub fn get_total_revenue(&self) -> u64 {
        self.total_revenue
    }

    pub fn get_average_revenue(&self) -> f64 {
        self.average_revenue
    }

    pub fn get_report_date(&self) -> &str {
        &self.report_date
    }
}
