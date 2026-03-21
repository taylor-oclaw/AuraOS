extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DocumentSignatureRequest {
    document_id: String,
    user_id: String,
    signature: Option<Vec<u8>>,
    status: String,
    timestamp: u64,
}

impl DocumentSignatureRequest {
    pub fn new(document_id: String, user_id: String) -> Self {
        DocumentSignatureRequest {
            document_id,
            user_id,
            signature: None,
            status: String::from("Pending"),
            timestamp: 0,
        }
    }

    pub fn set_signature(&mut self, signature: Vec<u8>) {
        self.signature = Some(signature);
        self.status = String::from("Signed");
    }

    pub fn get_document_id(&self) -> &str {
        &self.document_id
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }

    pub fn is_signed(&self) -> bool {
        self.status == "Signed"
    }

    pub fn update_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }
}
