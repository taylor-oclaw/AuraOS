extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct AIContext {
    context_id: u32,
    data: Vec<u8>,
    metadata: String,
}

impl AIContext {
    pub fn new(context_id: u32, initial_data: &[u8], metadata: &str) -> Self {
        AIContext {
            context_id,
            data: initial_data.to_vec(),
            metadata: String::from(metadata),
        }
    }

    pub fn get_context_id(&self) -> u32 {
        self.context_id
    }

    pub fn set_metadata(&mut self, new_metadata: &str) {
        self.metadata = String::from(new_metadata);
    }

    pub fn append_data(&mut self, additional_data: &[u8]) {
        self.data.extend_from_slice(additional_data);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_context() {
        let mut context = AIContext::new(1, b"initial", "metadata");
        assert_eq!(context.get_context_id(), 1);
        assert_eq!(context.get_data(), b"initial");
        assert_eq!(context.metadata, "metadata");

        context.set_metadata("new_metadata");
        assert_eq!(context.metadata, "new_metadata");

        context.append_data(b"_data");
        assert_eq!(context.get_data(), b"initial_data");

        context.clear_data();
        assert_eq!(context.get_data(), b"");
    }
}
