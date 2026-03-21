extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Snapshot {
    pub id: u64,
    pub name: String,
    pub timestamp: u64,
    pub size_bytes: u64,
    pub file_count: u64,
    pub auto_created: bool
}

pub enum RestoreStatus {
    Idle,
    InProgress(u8),
    Complete,
    Failed(String)
}

pub struct BackupManager {
    pub snapshots: Vec<Snapshot>,
    pub next_id: u64,
    pub auto_interval_secs: u64,
    pub max_snapshots: usize,
    pub restore_status: RestoreStatus
}

impl BackupManager {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            next_id: 1,
            auto_interval_secs: 3600,
            max_snapshots: 24,
            restore_status: RestoreStatus::Idle
        }
    }

    pub fn create_snapshot(&mut self, name: &str, auto: bool) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.snapshots.push(Snapshot {
            id,
            name: String::from(name),
            timestamp: 0,
            size_bytes: 0,
            file_count: 0,
            auto_created: auto
        });
        if self.snapshots.len() > self.max_snapshots {
            self.snapshots.remove(0);
        }
        id
    }

    pub fn list_snapshots(&self) -> &[Snapshot] {
        &self.snapshots
    }

    pub fn delete_snapshot(&mut self, id: u64) {
        self.snapshots.retain(|s| s.id != id);
    }

    pub fn start_restore(&mut self, snapshot_id: u64) -> bool {
        if self.snapshots.iter().any(|s| s.id == snapshot_id) {
            self.restore_status = RestoreStatus::InProgress(0);
            true
        } else {
            false
        }
    }

    pub fn restore_progress(&self) -> u8 {
        match &self.restore_status {
            RestoreStatus::InProgress(pct) => *pct,
            RestoreStatus::Complete => 100,
            _ => 0
        }
    }

    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }
}
