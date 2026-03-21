extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct ContractRenewal {
    vendor_name: String,
    contract_id: u64,
    renewal_date: u64, // Unix timestamp
    terms: Vec<String>,
    status: String,
}

impl ContractRenewal {
    pub fn new(vendor_name: &str, contract_id: u64, renewal_date: u64) -> Self {
        ContractRenewal {
            vendor_name: String::from(vendor_name),
            contract_id,
            renewal_date,
            terms: Vec::new(),
            status: String::from("Active"),
        }
    }

    pub fn add_term(&mut self, term: &str) {
        self.terms.push(String::from(term));
    }

    pub fn get_vendor_name(&self) -> &str {
        &self.vendor_name
    }

    pub fn get_contract_id(&self) -> u64 {
        self.contract_id
    }

    pub fn get_renewal_date(&self) -> u64 {
        self.renewal_date
    }

    pub fn get_terms(&self) -> &Vec<String> {
        &self.terms
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn renew_contract(&mut self, new_renewal_date: u64) {
        self.renewal_date = new_renewal_date;
        self.status = String::from("Renewed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let contract = ContractRenewal::new("Vendor A", 12345, 1672531200);
        assert_eq!(contract.vendor_name, "Vendor A");
        assert_eq!(contract.contract_id, 12345);
        assert_eq!(contract.renewal_date, 1672531200);
        assert_eq!(contract.terms.len(), 0);
        assert_eq!(contract.status, "Active");
    }

    #[test]
    fn test_add_term() {
        let mut contract = ContractRenewal::new("Vendor A", 12345, 1672531200);
        contract.add_term("Term 1");
        assert_eq!(contract.terms.len(), 1);
        assert_eq!(contract.terms[0], "Term 1");
    }

    #[test]
    fn test_renew_contract() {
        let mut contract = ContractRenewal::new("Vendor A", 12345, 1672531200);
        contract.renew_contract(1675209600);
        assert_eq!(contract.renewal_date, 1675209600);
        assert_eq!(contract.status, "Renewed");
    }
}
