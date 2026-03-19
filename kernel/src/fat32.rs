extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct BiosParameterBlock {
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub num_fats: u8,
    pub total_sectors: u32,
    pub fat_size: u32,
    pub root_cluster: u32
}

pub struct DirEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: u32,
    pub first_cluster: u32
}

pub struct Fat32 {
    pub bpb: BiosParameterBlock,
    pub fat: Vec<u32>
}

impl Fat32 {
    pub fn parse_bpb(sector: &[u8]) -> Option<BiosParameterBlock> {
        if sector.len() < 512 {
            return None;
        }
        let bytes_per_sector = u16::from_le_bytes([sector[11], sector[12]]);
        let sectors_per_cluster = sector[13];
        let reserved_sectors = u16::from_le_bytes([sector[14], sector[15]]);
        let num_fats = sector[16];
        let total_sectors = u32::from_le_bytes([sector[32], sector[33], sector[34], sector[35]]);
        let fat_size = u32::from_le_bytes([sector[36], sector[37], sector[38], sector[39]]);
        let root_cluster = u32::from_le_bytes([sector[44], sector[45], sector[46], sector[47]]);

        Some(BiosParameterBlock {
            bytes_per_sector,
            sectors_per_cluster,
            reserved_sectors,
            num_fats,
            total_sectors,
            fat_size,
            root_cluster
        })
    }

    pub fn read_fat_entry(&self, cluster: u32) -> u32 {
        if (cluster as usize) >= self.fat.len() {
            return 0xFFFFFFFF;
        }
        self.fat[cluster as usize]
    }

    pub fn parse_dir_entry(data: &[u8]) -> Option<DirEntry> {
        if data.len() < 32 {
            return None;
        }
        let name = {let s = String::from_utf8_lossy(&data[0..11]); let mut name = String::from(s.as_ref()); while name.ends_with(' ') { name.pop(); } name};
        let is_dir = data[11] == 0x10 || (data[11] & 0x10) != 0;
        let size = u32::from_le_bytes([data[28], data[29], data[30], data[31]]);
        let first_cluster = u32::from_le_bytes([data[26], data[27], 0, 0]);

        Some(DirEntry {
            name,
            is_dir,
            size,
            first_cluster
        })
    }

    pub fn list_dir(&self, cluster: u32, disk: &[u8]) -> Vec<DirEntry> {
        let mut entries = Vec::new();
        let mut current_cluster = cluster;
        loop {
            let offset = self.cluster_to_offset(current_cluster);
            for i in (0..self.bpb.bytes_per_sector as usize).step_by(32) {
                if i + 32 > disk.len() - offset {
                    break;
                }
                if let Some(entry) = Self::parse_dir_entry(&disk[offset + i..offset + i + 32]) {
                    entries.push(entry);
                }
            }
            current_cluster = self.read_fat_entry(current_cluster);
            if current_cluster >= 0x0FFFFFF8 {
                break;
            }
        }
        entries
    }

    pub fn cluster_to_offset(&self, cluster: u32) -> usize {
        ((cluster - 2) * self.bpb.sectors_per_cluster as u32 * self.bpb.bytes_per_sector as u32 + 
         (self.bpb.reserved_sectors as u32 + self.bpb.num_fats as u32 * self.bpb.fat_size) * self.bpb.bytes_per_sector as u32) as usize
    }
}
