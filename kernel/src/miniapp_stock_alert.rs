extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("MiniApp Stock Alert Module Loaded");
    0
}

pub struct StockAlert {
    stock_symbol: String,
    alert_price: f64,
    current_price: f64,
    alerts_sent: Vec<String>,
}

impl StockAlert {
    pub fn new(stock_symbol: &str, alert_price: f64) -> Self {
        StockAlert {
            stock_symbol: String::from(stock_symbol),
            alert_price,
            current_price: 0.0,
            alerts_sent: Vec::new(),
        }
    }

    pub fn update_current_price(&mut self, price: f64) {
        self.current_price = price;
        if self.should_send_alert() {
            self.send_alert();
        }
    }

    pub fn get_stock_symbol(&self) -> &str {
        &self.stock_symbol
    }

    pub fn get_alert_price(&self) -> f64 {
        self.alert_price
    }

    pub fn get_current_price(&self) -> f64 {
        self.current_price
    }

    pub fn get_alerts_sent(&self) -> &[String] {
        &self.alerts_sent
    }

    fn should_send_alert(&self) -> bool {
        self.current_price >= self.alert_price
    }

    fn send_alert(&mut self) {
        let alert_message = format!("Alert: {} has reached or exceeded the price of {}", self.stock_symbol, self.alert_price);
        self.alerts_sent.push(alert_message);
        // Simulate sending an alert (e.g., logging to a file or displaying on screen)
        println!("{}", self.alerts_sent.last().unwrap());
    }
}
