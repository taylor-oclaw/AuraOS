//! OS Update System — A/B Partitions
//! Safe updates with automatic rollback.

extern crate alloc;
use alloc::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Partition {
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateState {
    Idle,
    Downloading,
    Installing,
    Verifying,
    ReadyToReboot,
    RollingBack,
}

/// Boot record stored on disk
#[derive(Debug, Clone)]
pub struct BootRecord {
    pub active_partition: Partition,
    pub boot_count: u32,
    pub max_boot_tries: u8,
    pub last_good_partition: Partition,
    pub update_pending: bool,
    pub current_version: String,
    pub pending_version: String,
}

pub struct UpdateManager {
    pub state: UpdateState,
    pub boot_record: BootRecord,
}

impl UpdateManager {
    pub fn new(version: &str) -> Self {
        UpdateManager {
            state: UpdateState::Idle,
            boot_record: BootRecord {
                active_partition: Partition::A,
                boot_count: 0,
                max_boot_tries: 3,
                last_good_partition: Partition::A,
                update_pending: false,
                current_version: String::from(version),
                pending_version: String::new(),
            },
        }
    }

    /// Call on every boot — checks if we need to rollback
    pub fn on_boot(&mut self) {
        self.boot_record.boot_count += 1;

        if self.boot_record.update_pending {
            if self.boot_record.boot_count > self.boot_record.max_boot_tries as u32 {
                // Too many failed boots — rollback
                self.rollback();
            }
        }
    }

    /// Mark current boot as successful
    pub fn commit(&mut self) {
        self.boot_record.boot_count = 0;
        self.boot_record.last_good_partition = self.boot_record.active_partition;
        self.boot_record.update_pending = false;
        self.state = UpdateState::Idle;
    }

    /// Start an update
    pub fn start_update(&mut self, version: &str) {
        self.boot_record.pending_version = String::from(version);
        self.state = UpdateState::Downloading;
    }

    /// Mark update as ready (downloaded + verified)
    pub fn mark_ready(&mut self) {
        self.boot_record.update_pending = true;
        self.boot_record.boot_count = 0;
        // Switch to the other partition on next boot
        self.boot_record.active_partition = match self.boot_record.active_partition {
            Partition::A => Partition::B,
            Partition::B => Partition::A,
        };
        self.state = UpdateState::ReadyToReboot;
    }

    /// Rollback to last good partition
    pub fn rollback(&mut self) {
        self.boot_record.active_partition = self.boot_record.last_good_partition;
        self.boot_record.update_pending = false;
        self.boot_record.boot_count = 0;
        self.state = UpdateState::RollingBack;
    }

    pub fn current_version(&self) -> &str {
        &self.boot_record.current_version
    }

    pub fn is_update_pending(&self) -> bool {
        self.boot_record.update_pending
    }
}
