extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

struct Donation {
    donor_name: String,
    amount: u32,
    message: String,
}

impl Donation {
    pub fn new(donor_name: &str, amount: u32, message: &str) -> Self {
        Donation {
            donor_name: String::from(donor_name),
            amount,
            message: String::from(message),
        }
    }

    pub fn get_donor_name(&self) -> &str {
        &self.donor_name
    }

    pub fn get_amount(&self) -> u32 {
        self.amount
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn update_amount(&mut self, new_amount: u32) {
        self.amount = new_amount;
    }

    pub fn append_to_message(&mut self, additional_message: &str) {
        self.message.push_str(additional_message);
    }
}

struct CharityDonationGift {
    donations: Vec<Donation>,
}

impl CharityDonationGift {
    pub fn new() -> Self {
        CharityDonationGift {
            donations: Vec::new(),
        }
    }

    pub fn add_donation(&mut self, donation: Donation) {
        self.donations.push(donation);
    }

    pub fn get_total_donations(&self) -> u32 {
        self.donations.iter().map(|d| d.get_amount()).sum()
    }

    pub fn find_donation_by_donor(&self, donor_name: &str) -> Option<&Donation> {
        self.donations.iter().find(|d| d.get_donor_name() == donor_name)
    }

    pub fn list_all_donors(&self) -> Vec<String> {
        self.donations.iter().map(|d| d.get_donor_name().to_string()).collect()
    }
}
