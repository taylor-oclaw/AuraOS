extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct FinanceCryptoTrack {
    assets: Vec<String>,
    prices: Vec<f64>,
    volumes: Vec<u64>,
}

impl FinanceCryptoTrack {
    pub fn new() -> Self {
        FinanceCryptoTrack {
            assets: Vec::new(),
            prices: Vec::new(),
            volumes: Vec::new(),
        }
    }

    pub fn add_asset(&mut self, asset_name: &str, price: f64, volume: u64) {
        self.assets.push(String::from(asset_name));
        self.prices.push(price);
        self.volumes.push(volume);
    }

    pub fn get_price(&self, asset_name: &str) -> Option<f64> {
        if let Some(index) = self.assets.iter().position(|a| a == asset_name) {
            Some(self.prices[index])
        } else {
            None
        }
    }

    pub fn get_volume(&self, asset_name: &str) -> Option<u64> {
        if let Some(index) = self.assets.iter().position(|a| a == asset_name) {
            Some(self.volumes[index])
        } else {
            None
        }
    }

    pub fn update_price(&mut self, asset_name: &str, new_price: f64) -> bool {
        if let Some(index) = self.assets.iter().position(|a| a == asset_name) {
            self.prices[index] = new_price;
            true
        } else {
            false
        }
    }

    pub fn update_volume(&mut self, asset_name: &str, new_volume: u64) -> bool {
        if let Some(index) = self.assets.iter().position(|a| a == asset_name) {
            self.volumes[index] = new_volume;
            true
        } else {
            false
        }
    }

    pub fn list_assets(&self) -> Vec<String> {
        self.assets.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finance_crypto_track() {
        let mut tracker = FinanceCryptoTrack::new();
        tracker.add_asset("BTC", 50000.0, 1000);
        tracker.add_asset("ETH", 3000.0, 2000);

        assert_eq!(tracker.get_price("BTC"), Some(50000.0));
        assert_eq!(tracker.get_volume("ETH"), Some(2000));

        assert!(tracker.update_price("BTC", 51000.0));
        assert!(tracker.update_volume("ETH", 2100));

        assert_eq!(tracker.get_price("BTC"), Some(51000.0));
        assert_eq!(tracker.get_volume("ETH"), Some(2100));

        let assets = tracker.list_assets();
        assert_eq!(assets, vec![String::from("BTC"), String::from("ETH")]);
    }
}
