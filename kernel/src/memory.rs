use bootloader_api::info::{MemoryRegion, MemoryRegionKind};

pub fn total_usable_memory(regions: &[MemoryRegion]) -> u64 {
    regions
        .iter()
        .filter(|r| r.kind == MemoryRegionKind::Usable)
        .map(|r| r.end - r.start)
        .sum()
}
