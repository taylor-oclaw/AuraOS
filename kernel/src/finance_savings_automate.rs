extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct SavingsAccount {
    account_holder: String,
    balance: u64,
    transactions: Vec<(String, u64)>,
}

impl SavingsAccount {
    pub fn new(account_holder: &str) -> Self {
        SavingsAccount {
            account_holder: String::from(account_holder),
            balance: 0,
            transactions: Vec::new(),
        }
    }

    pub fn deposit(&mut self, amount: u64) {
        if amount > 0 {
            self.balance += amount;
            self.transactions.push((String::from("deposit"), amount));
        }
    }

    pub fn withdraw(&mut self, amount: u64) -> bool {
        if amount > 0 && amount <= self.balance {
            self.balance -= amount;
            self.transactions.push((String::from("withdrawal"), amount));
            true
        } else {
            false
        }
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }

    pub fn get_transactions(&self) -> &Vec<(String, u64)> {
        &self.transactions
    }

    pub fn account_summary(&self) -> String {
        let mut summary = String::from("info");
        for (transaction_type, amount) in &self.transactions {
            summary.push_str(&String::from("info"));
        }
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_savings_account() {
        let mut account = SavingsAccount::new("John Doe");
        assert_eq!(account.get_balance(), 0);
        assert_eq!(account.withdraw(100), false);

        account.deposit(500);
        assert_eq!(account.get_balance(), 500);
        assert_eq!(account.withdraw(200), true);
        assert_eq!(account.get_balance(), 300);

        let transactions = account.get_transactions();
        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0], (String::from("deposit"), 500));
        assert_eq!(transactions[1], (String::from("withdrawal"), 200));

        let summary = account.account_summary();
        assert!(summary.contains("Account Holder: John Doe"));
        assert!(summary.contains("Balance: 300"));
        assert!(summary.contains("deposit - 500"));
        assert!(summary.contains("withdrawal - 200"));
    }
}
