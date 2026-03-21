extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut tracker = DeliveryTracker::new();
    tracker.add_delivery("Order123", "New York");
    tracker.add_delivery("Order456", "Los Angeles");
    tracker.update_status("Order123", "In Transit");
    tracker.update_status("Order456", "Delivered");
    let status = tracker.get_status("Order123").unwrap();
    loop {}
}

pub struct DeliveryTracker {
    deliveries: Vec<Delivery>,
}

impl DeliveryTracker {
    pub fn new() -> Self {
        DeliveryTracker {
            deliveries: Vec::new(),
        }
    }

    pub fn add_delivery(&mut self, order_id: &str, destination: &str) {
        let delivery = Delivery {
            order_id: String::from(order_id),
            destination: String::from(destination),
            status: String::from("Pending"),
        };
        self.deliveries.push(delivery);
    }

    pub fn update_status(&mut self, order_id: &str, new_status: &str) {
        if let Some(delivery) = self.deliveries.iter_mut().find(|d| d.order_id == order_id) {
            delivery.status = String::from(new_status);
        }
    }

    pub fn get_status(&self, order_id: &str) -> Option<&String> {
        self.deliveries
            .iter()
            .find(|d| d.order_id == order_id)
            .map(|d| &d.status)
    }

    pub fn list_all_deliveries(&self) -> Vec<String> {
        self.deliveries
            .iter()
            .map(|d| String::from("info"))
            .collect()
    }
}

struct Delivery {
    order_id: String,
    destination: String,
    status: String,
}
