extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct MarketplaceRevenueSplit {
    sellers: Vec<Seller>,
}

impl MarketplaceRevenueSplit {
    pub fn new() -> Self {
        MarketplaceRevenueSplit {
            sellers: Vec::new(),
        }
    }

    pub fn add_seller(&mut self, name: String, commission_rate: f32) {
        let seller = Seller {
            name,
            commission_rate,
            total_sales: 0.0,
        };
        self.sellers.push(seller);
    }

    pub fn record_sale(&mut self, seller_name: &str, sale_amount: f32) -> Result<(), String> {
        if let Some(seller) = self.sellers.iter_mut().find(|s| s.name == seller_name) {
            seller.total_sales += sale_amount;
            Ok(())
        } else {
            Err(String::from("Seller not found"))
        }
    }

    pub fn get_seller_total_sales(&self, seller_name: &str) -> Result<f32, String> {
        if let Some(seller) = self.sellers.iter().find(|s| s.name == seller_name) {
            Ok(seller.total_sales)
        } else {
            Err(String::from("Seller not found"))
        }
    }

    pub fn calculate_commission(&self, seller_name: &str) -> Result<f32, String> {
        if let Some(seller) = self.sellers.iter().find(|s| s.name == seller_name) {
            Ok(seller.total_sales * seller.commission_rate)
        } else {
            Err(String::from("Seller not found"))
        }
    }

    pub fn total_revenue(&self) -> f32 {
        self.sellers.iter().map(|s| s.total_sales).sum()
    }
}

#[derive(Debug)]
struct Seller {
    name: String,
    commission_rate: f32,
    total_sales: f32,
}
