use bootloader_api::info::MemoryRegionKind;
use x86_64::PhysAddr;

/// Returns the total amount of usable physical memory.
pub fn total_usable_memory(regions: &[bootloader_api::info::MemoryRegion]) -> u64 {
    regions
        .iter()
        .filter(|r| r.kind == MemoryRegionKind::Usable)
        .map(|r| r.end - r.start)
        .sum()
}
