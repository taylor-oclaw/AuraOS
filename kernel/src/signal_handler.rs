extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum Signal { Kill, Term, Stop, Cont, Hup, Int, Usr1, Usr2, Alarm(u64), Custom(u8) }

pub struct SignalEntry {
    pub signal: Signal,
    pub target_pid: u64,
    pub from_pid: u64,
    pub timestamp: u64,
    pub delivered: bool,
}

pub struct SignalHandler {
    pub pending: Vec<SignalEntry>,
    pub masked: Vec<u8>,
    pub total_delivered: u64,
}

impl SignalHandler {
    pub fn new() -> Self {
        Self { pending: Vec::new(), masked: Vec::new(), total_delivered: 0 }
    }

    pub fn send_signal(&mut self, signal: Signal, target: u64, from: u64) {
        self.pending.push(SignalEntry {
            signal, target_pid: target, from_pid: from, timestamp: 0, delivered: false,
        });
    }

    pub fn deliver(&mut self, pid: u64) -> Option<&Signal> {
        if let Some(entry) = self.pending.iter_mut().find(|e| e.target_pid == pid && !e.delivered) {
            entry.delivered = true;
            self.total_delivered += 1;
            Some(&entry.signal)
        } else { None }
    }

    pub fn mask_signal(&mut self, sig_num: u8) {
        if !self.masked.contains(&sig_num) { self.masked.push(sig_num); }
    }

    pub fn unmask_signal(&mut self, sig_num: u8) {
        self.masked.retain(|s| *s != sig_num);
    }

    pub fn pending_for(&self, pid: u64) -> usize {
        self.pending.iter().filter(|e| e.target_pid == pid && !e.delivered).count()
    }

    pub fn total(&self) -> u64 { self.total_delivered }
}
