extern crate alloc;

use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;

const MAX_QUEUE_MESSAGES: usize = 64;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Data,
    Signal,
    Request,
    Response,
}

impl MessageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Data => "data",
            Self::Signal => "signal",
            Self::Request => "request",
            Self::Response => "response",
        }
    }

    pub fn as_string(&self) -> String {
        String::from(self.as_str())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Message {
    pub sender_pid: u32,
    pub receiver_pid: u32,
    pub msg_type: MessageType,
    pub payload: Vec<u8>,
    pub timestamp: u64,
}

pub struct MessageQueue {
    messages: VecDeque<Message>,
    max_messages: usize,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self {
            messages: VecDeque::new(),
            max_messages: MAX_QUEUE_MESSAGES,
        }
    }

    pub fn enqueue(&mut self, message: Message) -> bool {
        if self.messages.len() >= self.max_messages {
            return false;
        }

        self.messages.push_back(message);
        true
    }

    pub fn dequeue(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }

    pub fn peek(&self) -> Option<&Message> {
        self.messages.front()
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IpcManager {
    queues: Vec<(u32, MessageQueue)>,
}

impl IpcManager {
    pub fn new() -> Self {
        Self { queues: Vec::new() }
    }

    pub fn send(
        &mut self,
        from_pid: u32,
        to_pid: u32,
        msg_type: MessageType,
        payload: Vec<u8>,
        timestamp: u64,
    ) -> bool {
        if let Some(queue) = self.queue_mut(to_pid) {
            return queue.enqueue(Message {
                sender_pid: from_pid,
                receiver_pid: to_pid,
                msg_type,
                payload,
                timestamp,
            });
        }

        false
    }

    pub fn receive(&mut self, pid: u32) -> Option<Message> {
        self.queue_mut(pid).and_then(|queue| queue.dequeue())
    }

    pub fn peek(&self, pid: u32) -> Option<&Message> {
        self.queue(pid).and_then(|queue| queue.peek())
    }

    pub fn pending_count(&self, pid: u32) -> usize {
        self.queue(pid).map(|queue| queue.len()).unwrap_or(0)
    }

    pub fn create_queue(&mut self, pid: u32) {
        if self.queue(pid).is_none() {
            self.queues.push((pid, MessageQueue::new()));
        }
    }

    pub fn destroy_queue(&mut self, pid: u32) {
        if let Some(index) = self
            .queues
            .iter()
            .position(|(queue_pid, _)| *queue_pid == pid)
        {
            self.queues.remove(index);
        }
    }

    fn queue(&self, pid: u32) -> Option<&MessageQueue> {
        self.queues
            .iter()
            .find(|(queue_pid, _)| *queue_pid == pid)
            .map(|(_, queue)| queue)
    }

    fn queue_mut(&mut self, pid: u32) -> Option<&mut MessageQueue> {
        self.queues
            .iter_mut()
            .find(|(queue_pid, _)| *queue_pid == pid)
            .map(|(_, queue)| queue)
    }
}

impl Default for IpcManager {
    fn default() -> Self {
        Self::new()
    }
}
