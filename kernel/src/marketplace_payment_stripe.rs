extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct StripePayment {
    api_key: String,
    customer_id: Option<String>,
    payment_intent_id: Option<String>,
}

impl StripePayment {
    pub fn new(api_key: &str) -> Self {
        StripePayment {
            api_key: String::from(api_key),
            customer_id: None,
            payment_intent_id: None,
        }
    }

    pub fn set_customer_id(&mut self, customer_id: &str) {
        self.customer_id = Some(String::from(customer_id));
    }

    pub fn create_payment_intent(&mut self, amount: u32, currency: &str) -> Result<(), String> {
        // Simulate creating a payment intent
        if amount == 0 || currency.is_empty() {
            return Err(String::from("Invalid amount or currency"));
        }
        self.payment_intent_id = Some(String::from("pi_1234567890"));
        Ok(())
    }

    pub fn confirm_payment(&mut self) -> Result<(), String> {
        // Simulate confirming a payment
        if let Some(payment_intent_id) = &self.payment_intent_id {
            return Ok(());
        }
        Err(String::from("Payment intent not created"))
    }

    pub fn capture_payment(&mut self) -> Result<(), String> {
        // Simulate capturing a payment
        if let Some(payment_intent_id) = &self.payment_intent_id {
            return Ok(());
        }
        Err(String::from("Payment intent not created"))
    }

    pub fn cancel_payment(&mut self) -> Result<(), String> {
        // Simulate cancelling a payment
        if let Some(payment_intent_id) = &self.payment_intent_id {
            self.payment_intent_id = None;
            return Ok(());
        }
        Err(String::from("Payment intent not created"))
    }

    pub fn get_payment_status(&self) -> Result<String, String> {
        // Simulate getting payment status
        if let Some(payment_intent_id) = &self.payment_intent_id {
            return Ok(String::from("succeeded"));
        }
        Err(String::from("Payment intent not created"))
    }
}
