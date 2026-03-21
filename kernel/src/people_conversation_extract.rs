extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleConversationExtract {
    conversations: Vec<String>,
}

impl PeopleConversationExtract {
    pub fn new() -> Self {
        PeopleConversationExtract {
            conversations: Vec::new(),
        }
    }

    pub fn add_conversation(&mut self, conversation: String) {
        self.conversations.push(conversation);
    }

    pub fn get_conversation_count(&self) -> usize {
        self.conversations.len()
    }

    pub fn get_all_conversations(&self) -> Vec<String> {
        self.conversations.clone()
    }

    pub fn find_conversation_by_keyword(&self, keyword: &str) -> Option<Vec<String>> {
        let mut matching_conversations = Vec::new();
        for conversation in &self.conversations {
            if conversation.contains(keyword) {
                matching_conversations.push(conversation.clone());
            }
        }
        if matching_conversations.is_empty() {
            None
        } else {
            Some(matching_conversations)
        }
    }

    pub fn remove_conversation(&mut self, index: usize) -> Option<String> {
        if index < self.conversations.len() {
            Some(self.conversations.remove(index))
        } else {
            None
        }
    }
}
