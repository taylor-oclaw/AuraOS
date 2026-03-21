extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn gift_gift_card_manage_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn gift_gift_card_manage_exit() {
    // Cleanup logic for the module
}

pub struct GiftCardManager {
    cards: Vec<GiftCard>,
}

impl GiftCardManager {
    pub fn new() -> Self {
        GiftCardManager { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: GiftCard) {
        self.cards.push(card);
    }

    pub fn remove_card_by_id(&mut self, card_id: &str) -> Option<GiftCard> {
        let position = self.cards.iter().position(|c| c.id == card_id);
        match position {
            Some(pos) => Some(self.cards.remove(pos)),
            None => None,
        }
    }

    pub fn get_card_by_id(&self, card_id: &str) -> Option<&GiftCard> {
        self.cards.iter().find(|&c| c.id == card_id)
    }

    pub fn list_all_cards(&self) -> Vec<&GiftCard> {
        self.cards.iter().collect()
    }
}

pub struct GiftCard {
    id: String,
    amount: u32,
    is_active: bool,
}

impl GiftCard {
    pub fn new(id: String, amount: u32) -> Self {
        GiftCard { id, amount, is_active: true }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn get_amount(&self) -> u32 {
        self.amount
    }

    pub fn set_amount(&mut self, amount: u32) {
        self.amount = amount;
    }
}
