extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

pub struct ElfHeader {
    pub entry_point: u64,
    pub program_header_offset: u64,
    pub section_header_offset: u64,
    pub num_program_headers: u16,
    pub num_section_headers: u16
}

pub struct ProgramHeader {
    pub vaddr: u64,
    pub paddr: u64,
    pub file_size: u64,
    pub mem_size: u64,
    pub flags: u32,
    pub segment_type: u32
}

pub struct ElfLoader;

impl ElfLoader {
    pub fn parse_header(data: &[u8]) -> Option<ElfHeader> {
        if data.len() < 0x40 || &data[0..4] != b"\x7FELF" {
            return None;
        }

        let class = data[4];
        if class != 2 { // ELF64
            return None;
        }

        Some(ElfHeader {
            entry_point: u64::from_le_bytes(data[0x18..0x20].try_into().unwrap()),
            program_header_offset: u64::from_le_bytes(data[0x20..0x28].try_into().unwrap()),
            section_header_offset: u64::from_le_bytes(data[0x28..0x30].try_into().unwrap()),
            num_program_headers: u16::from_le_bytes(data[0x38..0x3A].try_into().unwrap()),
            num_section_headers: u16::from_le_bytes(data[0x3C..0x3E].try_into().unwrap())
        })
    }

    pub fn parse_program_headers(data: &[u8], header: &ElfHeader) -> Vec<ProgramHeader> {
        let mut program_headers = Vec::new();
        let offset = header.program_header_offset as usize;
        let entry_size = 0x38; // Size of a 64-bit ELF program header

        for i in 0..header.num_program_headers {
            let start = offset + (i as usize) * entry_size;
            if start + entry_size > data.len() {
                break;
            }

            program_headers.push(ProgramHeader {
                vaddr: u64::from_le_bytes(data[start+0x08..start+0x10].try_into().unwrap()),
                paddr: u64::from_le_bytes(data[start+0x10..start+0x18].try_into().unwrap()),
                file_size: u64::from_le_bytes(data[start+0x20..start+0x28].try_into().unwrap()),
                mem_size: u64::from_le_bytes(data[start+0x28..start+0x30].try_into().unwrap()),
                flags: u32::from_le_bytes(data[start+0x30..start+0x34].try_into().unwrap()),
                segment_type: u32::from_le_bytes(data[start+0x00..start+0x04].try_into().unwrap())
            });
        }

        program_headers
    }

    pub fn is_valid(data: &[u8]) -> bool {
        data.len() >= 0x40 && &data[0..4] == b"\x7FELF" && data[4] == 2 // ELF64
    }

    pub fn entry_point(data: &[u8]) -> Option<u64> {
        if let Some(header) = ElfLoader::parse_header(data) {
            Some(header.entry_point)
        } else {
            None
        }
    }
}
