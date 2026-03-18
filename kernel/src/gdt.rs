use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

static TSS: spin::Lazy<TaskStateSegment> = spin::Lazy::new(|| {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 20; // 80KB stack
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
        let stack_start = VirtAddr::from_ptr(unsafe { core::ptr::addr_of!(STACK) });
        stack_start + STACK_SIZE as u64
    };
    // Also set up a proper privilege stack (RSP0) for interrupts
    tss.privilege_stack_table[0] = {
        const STACK_SIZE: usize = 4096 * 20; // 80KB
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
        let stack_start = VirtAddr::from_ptr(unsafe { core::ptr::addr_of!(STACK) });
        stack_start + STACK_SIZE as u64
    };
    tss
});

static GDT: spin::Lazy<(GlobalDescriptorTable, Selectors)> = spin::Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let code = gdt.append(Descriptor::kernel_code_segment());
    let tss = gdt.append(Descriptor::tss_segment(&TSS));
    (gdt, Selectors { code, tss })
});

struct Selectors { code: SegmentSelector, tss: SegmentSelector }

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code);
        load_tss(GDT.1.tss);
    }
}
