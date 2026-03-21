extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_chargeback_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_chargeback_exit() {
    // Cleanup logic for the module
}

pub struct MeshChargeback {
    transactions: Vec<Transaction>,
}

impl MeshChargeback {
    pub fn new() -> Self {
        MeshChargeback {
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn total_amount(&self) -> u64 {
        self.transactions.iter().map(|t| t.amount).sum()
    }

    pub fn find_transaction_by_id(&self, id: u32) -> Option<&Transaction> {
        self.transactions.iter().find(|t| t.id == id)
    }
}

pub struct Transaction {
    id: u32,
    amount: u64,
    description: String,
}

impl Transaction {
    pub fn new(id: u32, amount: u64, description: String) -> Self {
        Transaction { id, amount, description }
    }
}
