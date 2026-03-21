use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const KEYBOARD_IST_INDEX: u16 = 1;

static TSS: spin::Lazy<TaskStateSegment> = spin::Lazy::new(|| {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 20; // 80KB stack
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
        let stack_start = VirtAddr::from_ptr(unsafe { core::ptr::addr_of!(STACK) };
        stack_start + STACK_SIZE as u64
    };
    // IST1 — dedicated stack for keyboard interrupt
    tss.interrupt_stack_table[KEYBOARD_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 10; // 40KB
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
        let stack_start = VirtAddr::from_ptr(unsafe { core::ptr::addr_of!(STACK) };
        stack_start + STACK_SIZE as u64
    };
    tss
};

static GDT: spin::Lazy<(GlobalDescriptorTable, Selectors)> = spin::Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let code = gdt.add_entry(Descriptor::kernel_code_segment());
    let tss = gdt.add_entry(Descriptor::tss_segment(&TSS));
    (gdt, Selectors { code, tss }
};

struct Selectors { code: SegmentSelector, tss: SegmentSelector }

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code);
        load_tss(GDT.1.tss);
    }
)))))}
