extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod finance_deal_finder {
    use super::*;

    pub struct FinanceDealFinder {
        deals: Vec<String>,
    }

    impl FinanceDealFinder {
        pub fn new() -> Self {
            FinanceDealFinder { deals: Vec::new() }
        }

        pub fn add_deal(&mut self, deal: String) {
            self.deals.push(deal);
        }

        pub fn remove_deal(&mut self, index: usize) -> Option<String> {
            if index < self.deals.len() {
                Some(self.deals.remove(index))
            } else {
                None
            }
        }

        pub fn get_deal(&self, index: usize) -> Option<&String> {
            self.deals.get(index)
        }

        pub fn list_all_deals(&self) -> &Vec<String> {
            &self.deals
        }

        pub fn find_deal_by_keyword(&self, keyword: &str) -> Vec<&String> {
            self.deals.iter().filter(|deal| deal.contains(keyword)).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::finance_deal_finder::FinanceDealFinder;

    #[test]
    fn test_finance_deal_finder() {
        let mut finder = FinanceDealFinder::new();
        assert_eq!(finder.list_all_deals().len(), 0);

        finder.add_deal(String::from("Buy Bitcoin"));
        finder.add_deal(String::from("Sell Ethereum"));
        finder.add_deal(String::from("Invest in stocks"));

        assert_eq!(finder.list_all_deals().len(), 3);
        assert_eq!(finder.get_deal(1), Some(&String::from("Sell Ethereum")));

        let removed = finder.remove_deal(0);
        assert_eq!(removed, Some(String::from("Buy Bitcoin")));
        assert_eq!(finder.list_all_deals().len(), 2);

        let deals_with_keyword = finder.find_deal_by_keyword("invest");
        assert_eq!(deals_with_keyword.len(), 1);
        assert_eq!(deals_with_keyword[0], &String::from("Invest in stocks"));
    }
}
