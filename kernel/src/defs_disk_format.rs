extern crate alloc;
use alloc::vec::Vec;

pub const DEFS_MAGIC: [u8; 8] = [0x44, 0x45, 0x46, 0x53, 0x46, 0x53, 0x30, 0x31];
pub const BLOCK_SIZE: u32 = 4096;

pub struct DiskSuperblock {
    pub magic: [u8; 8],
    pub version: u32,
    pub total_blocks: u64,
    pub free_blocks: u64,
    pub total_inodes: u64,
    pub free_inodes: u64,
    pub block_size: u32,
    pub journal_start: u64,
    pub journal_blocks: u32,
    pub root_inode: u64,
    pub features: u64,
    pub uuid: [u8; 16],
    pub label: [u8; 64]
}

impl DiskSuperblock {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.magic);
        buf.extend_from_slice(&self.version.to_le_bytes());
        buf.extend_from_slice(&self.total_blocks.to_le_bytes());
        buf.extend_from_slice(&self.free_blocks.to_le_bytes());
        buf.extend_from_slice(&self.total_inodes.to_le_bytes());
        buf.extend_from_slice(&self.free_inodes.to_le_bytes());
        buf.extend_from_slice(&self.block_size.to_le_bytes());
        buf.extend_from_slice(&self.journal_start.to_le_bytes());
        buf.extend_from_slice(&self.journal_blocks.to_le_bytes());
        buf.extend_from_slice(&self.root_inode.to_le_bytes());
        buf.extend_from_slice(&self.features.to_le_bytes());
        buf.extend_from_slice(&self.uuid);
        buf.extend_from_slice(&self.label);
        buf
    }

    pub fn deserialize(data: &[u8]) -> Option<Self> {
        if data.len() < 140 || &data[0..8] != DEFS_MAGIC {
            return None;
        }
        Some(Self {
            magic: DEFS_MAGIC,
            version: u32::from_le_bytes([data[8], data[9], data[10], data[11]]),
            total_blocks: u64::from_le_bytes([data[12], data[13], data[14], data[15], data[16], data[17], data[18], data[19]]),
            free_blocks: u64::from_le_bytes([data[20], data[21], data[22], data[23], data[24], data[25], data[26], data[27]]),
            total_inodes: u64::from_le_bytes([data[28], data[29], data[30], data[31], data[32], data[33], data[34], data[35]]),
            free_inodes: u64::from_le_bytes([data[36], data[37], data[38], data[39], data[40], data[41], data[42], data[43]]),
            block_size: u32::from_le_bytes([data[44], data[45], data[46], data[47]]),
            journal_start: u64::from_le_bytes([data[48], data[49], data[50], data[51], data[52], data[53], data[54], data[55]]),
            journal_blocks: u32::from_le_bytes([data[56], data[57], data[58], data[59]]),
            root_inode: u64::from_le_bytes([data[60], data[61], data[62], data[63], data[64], data[65], data[66], data[67]]),
            features: u64::from_le_bytes([data[68], data[69], data[70], data[71], data[72], data[73], data[74], data[75]]),
            uuid: {
                let mut u = [0u8; 16];
                u.copy_from_slice(&data[76..92]);
                u
            },
            label: {
                let mut l = [0u8; 64];
                l.copy_from_slice(&data[92..156]);
                l
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        self.magic == DEFS_MAGIC
    }
)}
