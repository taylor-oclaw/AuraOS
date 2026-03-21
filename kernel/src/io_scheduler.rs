extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum IoOperation {
    Read { block: u64, count: u32 },
    Write { block: u64, data: Vec<u8> },
    Flush,
    Trim { block: u64, count: u32 }
}

pub struct IoRequest {
    pub id: u64,
    pub operation: IoOperation,
    pub priority: u8,
    pub agent_id: u64,
    pub submitted_at: u64,
    pub completed: bool
}

pub struct IoScheduler {
    pub queue: Vec<IoRequest>,
    pub completed: Vec<u64>,
    pub next_id: u64,
    pub reads: u64,
    pub writes: u64,
    pub bytes_read: u64,
    pub bytes_written: u64
}

impl IoScheduler {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            completed: Vec::new(),
            next_id: 1,
            reads: 0,
            writes: 0,
            bytes_read: 0,
            bytes_written: 0
        }
    }

    pub fn submit(&mut self, op: IoOperation, priority: u8, agent: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.queue.push(IoRequest {
            id,
            operation: op,
            priority,
            agent_id: agent,
            submitted_at: 0,
            completed: false
        });
        id
    }

    pub fn process_next(&mut self) -> Option<u64> {
        self.queue.sort_by(|a, b| b.priority.cmp(&a.priority));
        if let Some(req) = self.queue.first_mut() {
            req.completed = true;
            let id = req.id;
            match &req.operation {
                IoOperation::Read { count, .. } => {
                    self.reads += 1;
                    self.bytes_read += *count as u64 * 4096;
                }
                IoOperation::Write { data, .. } => {
                    self.writes += 1;
                    self.bytes_written += data.len() as u64;
                }
                _ => {}
            }
            self.completed.push(id);
            self.queue.remove(0);
            Some(id)
        } else {
            None
        }
    }

    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }

    pub fn stats(&self) -> (u64, u64, u64, u64) {
        (self.reads, self.writes, self.bytes_read, self.bytes_written)
    }
}
