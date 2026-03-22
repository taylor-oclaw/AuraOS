#![no_std]
#![feature(allocator_api)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceCertification {
    pub sellers: Vec<String>,
    pub buyers: Vec<String>,
    pub products: Vec<String>,
}

impl MarketplaceCertification {
    pub fn new() -> Self {
        MarketplaceCertification {
            sellers: Vec::new(),
            buyers: Vec::new(),
            products: Vec::new(),
        }
    }

    pub fn add_seller(&mut self, seller: String) {
        self.sellers.push(seller);
    }

    pub fn add_buyer(&mut self, buyer: String) {
        self.buyers.push(buyer);
    }

    pub fn add_product(&mut self, product: String) {
        self.products.push(product);
    }

    pub fn get_sellers(&self) -> &Vec<String> {
        &self.sellers
    }

    pub fn get_buyers(&self) -> &Vec<String> {
        &self.buyers
    }
}